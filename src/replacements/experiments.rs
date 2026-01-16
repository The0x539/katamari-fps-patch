use super::*;

pub unsafe extern "C" fn gravity() {
    unsafe {
        asm! {
            "movss xmm5, dword ptr [rbx+0x1a0]",
            "mulss xmm5, dword ptr [rip+{dt}]",

            "xor edi, edi",
            "movss dword ptr [rbx+0x1c4], xmm0",
            "mov qword ptr [rbx+0x2f0], rdi",
            "mov dword ptr [rbx+0x2f8], edi",
            "mov dword ptr [rbx+0x2fc], {one}",

            dt = sym values::DT_TICKS,
            one = const 1_f32.to_bits(),
        }
    }
}

pub unsafe extern "C" fn gravitee() -> u64 {
    let k = unsafe {
        let k: *mut Katamari;
        asm!("", out("rcx") k);
        &mut *k
    };

    let p_idx = ps2::current_player_index();

    let prince = unsafe { &mut *ps2::prince_array(p_idx) };

    // trampoline
    k.spinning_in_place = prince.ouji_state.dash_stationary_spin;

    ps2::calculate_gravity(k);
    println!("{:?}", k.motion.gravity);
    // k.dual_a.external_velocity = k.dual_a.external_velocity * values::dt_ticks();

    // trampoline: prepare RAX
    ps2::current_stage() as u64
}

pub unsafe extern "C" fn good_night() {
    let k = unsafe {
        let k: *mut Katamari;
        asm!("mov rdi, rcx", out("rdi") k);
        &mut *k
    };

    let dividend = k.speed;
    let d = k.speed_fac_df * k.speed_fac_d * ps2::g_katamari_speed_d();
    let f = k.speed_fac_df * k.speed_fac_f * ps2::g_katamari_speed_f();

    let mut result: f32;
    if dividend < d {
        result = 1.0;
        if f < dividend {
            result = 1.0 - (dividend - f) / (d - f);
        }
    } else {
        result = 0.0;
    }

    result *= values::dt_ticks(); // IS THIS THE GUY
    // No, it doesn't seem like it. Damn.

    // TODO: get this to believe it's okay to leave a value in xmm10. I had to patch it in x64dbg.
    unsafe {
        asm! {
            "",
            in("xmm1") dividend,
            in("xmm0") d,
            in("xmm2") f,
            in("xmm10") result,
            in("rcx") k,
            in("rdi") k,
        }
    }
}

pub unsafe extern "win64" fn my_big_kahuna(k: *mut Katamari) {
    unsafe {
        // let mut direction = Vec4::W;

        (*k).x660 = Vec4::W;
        (*k).motion.effective = Vec4::W;

        let mut base_vel = (*k).motion.a + (*k).motion.downhill.xyz0();
        let mut vel = base_vel;

        if 0.0 < base_vel.len() {
            vel = base_vel;
            if !(*k).climbing {
                base_vel += (*k).motion.local_spin.xyz0();
                vel = base_vel;
            }
        }

        if (*k).speed_limit_check_1 != 0 && (*k).slope_state == 2 {
            let diff_speed = (*k).divisors.x * 3.0 * ps2::g_katamari_speed_f();
            if diff_speed < base_vel.len() {
                let mut direction = Vec4::ZERO;
                ps2::normalize(&mut direction, &base_vel.x0zw());
                vel = direction * diff_speed;
            }
        }

        (*k).motion.effective = vel;

        let foo = vel.x0zw();
        ps2::normalize(&mut (*k).motion.g, &foo);

        let bar = vel + (*k).motion.gravity;
        (*k).motion.h = bar;
        ps2::normalize(&mut (*k).motion.i, &bar.x0zw());

        // let mut b_var_2 = false;
        if !(*k).standing_on_prop {
            let mut b_var_2 = true;
            if ps2::multiplayer() {
                // I have no idea what the hell this code is
                b_var_2 = true;
                if (*ps2::prince_array((*k).player_index)).ouji_state.x17 {
                    b_var_2 = true;
                    if !(*k).airborne {
                        b_var_2 = false;
                    }
                }

                if (*k).climb_timer_possibly != 0 {
                    b_var_2 = true;
                    (*k).climb_timer_possibly -= 1;
                }
            }

            if ps2::game_mode() == 3 {
                (*k).position.y = -(-400.0 - (*k).radius_cm);
                // the instructions around here are relatively difficult to understand
                ps2::x25fb0(&(*k).motion.h.x0zw());
            } else if b_var_2 {
                if !(*k).climbing {
                    (*k).position += vel;
                    (*k).position += (*k).motion.gravity.xyz0();
                } else {
                    if !(*k).climb_height_reached {
                        (*k).position += vel.xyz0();
                    }
                    let foo = Vec4::ZERO;
                    ps2::climb_guy(&foo, k);
                }
            }

            if (*k).airborne {
                vel = (*k).motion.effective + (*k).motion.gravity;
            }
            (*k).speed = vel.len();
            ps2::update_katamari_measurements(k);
            ps2::katamari_update_spin(k, &vel.x0zw());
        } else {
            ps2::update_katamari_measurements(k);
            ps2::katamari_pivot(k);
        }

        // the weird loops that don't look like they do anything in the decomp

        (*k).x434 = (*k).speed;
        let before_deadzones = (*k).position.x0zw();
        ps2::apply_deadzones(&mut (*k).position, &before_deadzones, 0.001);

        let foo = (*k).speed / (*k).x80;
        (*k).x88 = foo;
        (*k).x800 = 1.0 - foo * ps2::val_x7b218();
        ps2::gravity_user_3(k);
        (*k).x764.0 = 0; // TODO: identify this field

        if ps2::multiplayer() {
            let p_idx = (*k).player_index;
            if (*ps2::camera_array(p_idx)).animation_mode == 5 {
                let mut n = (*k).radius_cm * 0.5;
                (*k).xc2 = true;
                let delta = (*k).prev_position - (*k).position;
                n = n.max(delta.len());

                (*k).timer_x3ad0 += 1;
                n = (*k).x3ac8 - n;
                (*k).x3ac8 = n;

                if n <= 0.0 {
                    if !std::ptr::addr_eq(ps2::katamari_array(p_idx), k) {
                        println!("katamari player index was not self-referential");
                    }

                    if (*k).xc2 {
                        (*k).xc2 = false;
                        (*k).x3ac8 = 0.0;
                        ps2::camera_set_view_mode(p_idx as i32, 6);
                        let prince = &mut *ps2::prince_array(p_idx);
                        prince.ouji_state.x19 = 0;
                        prince.ouji_state.dash_pending = false;
                        prince.ouji_state.dash_base_requirement_met = false;
                        prince.ouji_state.dash_spinning = false;
                        prince.ouji_state.dash_stationary_spin = false;
                        prince.dash_input_counter = 0;
                        prince.ouji_state.x16 = 0;
                        prince.ouji_state.x17 = false;
                    }
                }
            } else {
                (*k).xc2 = false;
            }
        }
        if ps2::current_stage() == 3 {
            (*k).xc0 = 1200.0 <= (*k).diameter_m;
        }
    }
}

pub unsafe extern "win64" fn wrong_big_kahuna(k: *mut Katamari) {
    // this was the wrong function
    unsafe {
        if !(*k).climb_height_reached {
            let climb_timer = (*k).climb_timer + 1;
            (*k).climb_timer = climb_timer;
            if (climb_timer as i32) < ps2::climb_limit() {
                ps2::terminate_climb(k)
            }
            (*k).climb_height_reached = true;
            return;
        }

        let limit_thing = (*k).position.y - (*k).x770;
        let mut limit_thing_2 = 0.0;
        if 0.0 <= limit_thing {
            limit_thing_2 = limit_thing;
        }

        if !(*k).climb_flag && (*k).measurement_guy_4 <= limit_thing_2 {
            let climb_timer = (*k).climb_timer + 1;
            (*k).climb_timer = climb_timer;
            if (climb_timer as i32) < ps2::climb_limit() {
                ps2::terminate_climb(k)
            }
            (*k).climb_height_reached = true;
            return;
        }

        (*k).another_climb_timer += 1;
        if (*k).another_climb_timer == 0x1e && !ps2::multiplayer() {}
    }
}
