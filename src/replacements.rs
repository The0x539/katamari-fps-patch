use crate::ps2;
use crate::types::{Katamari, Vec4};

#[unsafe(no_mangle)]
pub unsafe extern "win64" fn null_big_kahuna(_k: *mut Katamari) {
    println!("kahuna")
}

#[unsafe(no_mangle)]
pub unsafe extern "win64" fn my_big_kahuna(k: *mut Katamari) {
    unsafe {
        // let mut direction = Vec4::W;

        (*k).x660 = Vec4::W;
        (*k).dual_a.f = Vec4::W;

        let mut base_vel = (*k).dual_a.a + (*k).dual_a.l.xyz0();
        let mut vel = base_vel;

        if 0.0 < base_vel.len() {
            vel = base_vel;
            if !(*k).currently_climbing {
                base_vel += (*k).dual_a.friction.xyz0();
                vel = base_vel;
            }
        }

        if (*k).speed_limit_check_1 != 0 && (*k).speed_limit_check_2 == 2 {
            let diff_speed = (*k).divisors.x * 3.0 * ps2::g_katamari_speed_f();
            if diff_speed < base_vel.len() {
                let mut direction = Vec4::ZERO;
                ps2::normalize(&mut direction, &base_vel.x0zw());
                vel = direction * diff_speed;
            }
        }

        (*k).dual_a.f = vel;

        let foo = vel.x0zw();
        ps2::normalize(&mut (*k).dual_a.g, &foo);

        let bar = vel + (*k).dual_a.external_velocity;
        (*k).dual_a.h = bar;
        ps2::normalize(&mut (*k).dual_a.i, &bar.x0zw());

        // let mut b_var_2 = false;
        if !(*k).standing_on_prop {
            let mut b_var_2 = true;
            if ps2::multiplayer() {
                // I have no idea what the hell this code is
                b_var_2 = true;
                if (*ps2::prince_ptr((*k).player_index)).ouji_state.x17 {
                    b_var_2 = true;
                    if (*k).climbing_related_c == 0 {
                        b_var_2 = false;
                    }
                }

                if (*k).climb_timer_possibly != 0 {
                    b_var_2 = true;
                    (*k).climb_timer_possibly -= 1;
                }
            }

            if ps2::game_mode() == 3 {
                (*k).position.y = -(-400.0 - (*k).radius);
                // the instructions around here are relatively difficult to understand
                ps2::x25fb0(&(*k).dual_a.h.x0zw());
            } else if b_var_2 {
                if !(*k).currently_climbing {
                    (*k).position += vel;
                    (*k).position += (*k).dual_a.external_velocity.xyz0();
                } else {
                    if !(*k).climb_height_reached {
                        (*k).position += vel.xyz0();
                    }
                    let foo = Vec4::ZERO;
                    ps2::climb_guy(&foo, k);
                }
            }

            if (*k).climbing_related_c != 0 {
                vel = (*k).dual_a.f + (*k).dual_a.external_velocity;
            }
            (*k).guy_that_gets_divided = vel.len();
            ps2::update_katamari_measurements(k);
            ps2::katamari_physics_sub_sub1(k, &vel.x0zw());
        } else {
            ps2::update_katamari_measurements(k);
            ps2::katamari_physics_big_subroutine(k);
        }

        // the weird loops that don't look like they do anything in the decomp

        (*k).x434 = (*k).guy_that_gets_divided;
        let before_deadzones = (*k).position.x0zw();
        ps2::apply_deadzones(&mut (*k).position, &before_deadzones, 0.001);

        let foo = (*k).guy_that_gets_divided / (*k).x80;
        (*k).x88 = foo;
        (*k).x800 = 1.0 - foo * ps2::val_x7b218();
        ps2::gravity_user_3(k);
        (*k).x764.0 = 0; // TODO: identify this field

        if ps2::multiplayer() {
            let p_idx = (*k).player_index;
            if (*ps2::camera_ptr(p_idx)).animation_mode == 5 {
                let mut n = (*k).radius * 0.5;
                (*k).xc2 = true;
                let delta = (*k).position_b - (*k).position;
                n = n.max(delta.len());

                (*k).timer_x3ad0 += 1;
                n = (*k).x3ac8 - n;
                (*k).x3ac8 = n;

                if n <= 0.0 {
                    if !std::ptr::addr_eq(ps2::katamari_ptr(p_idx), k) {
                        println!("katamari player index was not self-referential");
                    }

                    if (*k).xc2 {
                        (*k).xc2 = false;
                        (*k).x3ac8 = 0.0;
                        ps2::set_player_animation_mode(p_idx as i32, 6);
                        let prince = &mut *ps2::prince_ptr(p_idx);
                        prince.ouji_state.x19 = 0;
                        prince.ouji_state.x0 = 0;
                        prince.ouji_state.x1 = 0;
                        prince.ouji_state.freeze_counter = false;
                        prince.ouji_state.x3 = 0;
                        prince.x47c.0 = 0; // TODO: identify this field or pair of fields
                        prince.ouji_state.x16 = 0;
                        prince.ouji_state.x17 = false;
                    }
                }
            } else {
                (*k).xc2 = false;
            }
        }
        if ps2::current_stage() == 3 {
            (*k).xc0 = 1200.0 <= (*k).diameter;
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
