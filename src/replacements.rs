use crate::ps2;
use crate::types::{Camera, Katamari, Prince, Vec4};

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

#[allow(non_snake_case, unused_assignments)]
pub unsafe extern "win64" fn raw_rewrite_big_kahuna(k: *mut Katamari) {
    unsafe {
        let mut bVar1: bool;
        let mut output: i64;
        // let mut lVar2: i64;
        let pIdx_: u64;
        // let mut lVar3: u64;
        let mut speed: f32;
        let mut y: f32;
        let deltaX: f32;
        let deltaZ: f32;
        let deltaY: f32;
        let velW: f32;
        let base_vel_x: f32;
        let mut diffSpeed: f32;
        let base_vel_y: f32;
        let base_vel_z: f32;
        let mut x0zw: Vec4 = Default::default();
        let mut direction: Vec4;
        let mut vel: Vec4 = Default::default();
        let w: f32;
        let w_: f32;
        let pIdx: u8;
        let prince: *mut Prince;

        let PRINCE: &mut [Prince; 2] = &mut *ps2::prince_ptr(0).cast();
        let CAMERAS: &mut [Camera; 2] = &mut *ps2::camera_ptr(0).cast();
        let KATAMARI: &mut [Katamari; 2] = &mut *ps2::katamari_ptr(0).cast();

        direction = Vec4::W;
        ((*k).x660).w = 1.0;
        ((*k).x660).x = 0.0;
        ((*k).x660).y = 0.0;
        ((*k).x660).z = 0.0;
        ((*k).dual_a).f.x = 0.0;
        ((*k).dual_a).f.y = 0.0;
        ((*k).dual_a).f.z = 0.0;
        ((*k).dual_a).f.w = 1.0;
        vel.w = ((*k).dual_a).a.w;
        base_vel_x = ((*k).dual_a).a.x + ((*k).dual_a).l.x;
        base_vel_y = ((*k).dual_a).a.y + ((*k).dual_a).l.y;
        base_vel_z = ((*k).dual_a).a.z + ((*k).dual_a).l.z;
        speed =
            (base_vel_y * base_vel_y + base_vel_x * base_vel_x + base_vel_z * base_vel_z).sqrt();
        vel.x = base_vel_x;
        vel.y = base_vel_y;
        vel.z = base_vel_z;
        if (0.0 < speed) && ((*k).currently_climbing == false) {
            // Apply friction deceleration
            vel.x = base_vel_x + ((*k).dual_a).friction.x;
            vel.y = base_vel_y + ((*k).dual_a).friction.y;
            vel.z = base_vel_z + ((*k).dual_a).friction.z;
        }
        if ((*k).speed_limit_check_1 != 0) && ((*k).speed_limit_check_2 == 2) {
            diffSpeed = ((*k).divisors).x * 3.0 * ps2::g_katamari_speed_f();
            speed = (vel.x * vel.x + vel.y * vel.y + vel.z * vel.z).sqrt();
            if diffSpeed < speed {
                y = vel.y;
                x0zw.x = vel.x;
                x0zw.w = vel.w;
                x0zw.z = vel.z;
                ps2::normalize(&mut direction, &x0zw);
                vel.x = direction.x * diffSpeed;
                vel.y = direction.y * diffSpeed;
                vel.z = direction.z * diffSpeed;
                vel.w = direction.w;
            }
        }
        ((*k).dual_a).f.x = vel.x;
        ((*k).dual_a).f.y = vel.y;
        ((*k).dual_a).f.z = vel.z;
        ((*k).dual_a).f.w = vel.w;
        y = vel.y;
        x0zw.x = vel.x;
        x0zw.w = vel.w;
        x0zw.z = vel.z;
        ps2::normalize(&mut ((*k).dual_a).g, &x0zw);
        output = &raw mut ((*k).dual_a).i as usize as i64;
        x0zw.x = vel.x + ((*k).dual_a).external_velocity.x;
        y = vel.y + ((*k).dual_a).external_velocity.y;
        x0zw.z = vel.z + ((*k).dual_a).external_velocity.z;
        x0zw.w = vel.w + ((*k).dual_a).external_velocity.w;
        ((*k).dual_a).h.x = x0zw.x;
        ((*k).dual_a).h.y = y;
        ((*k).dual_a).h.z = x0zw.z;
        ((*k).dual_a).h.w = x0zw.w;
        ps2::normalize(output as *mut _, &x0zw);
        if (*k).standing_on_prop == false {
            bVar1 = true;
            if ps2::multiplayer() == true {
                output = (*k).player_index as i64 * 0x518;
                bVar1 = true;
                if (PRINCE[(*k).player_index as usize].ouji_state.x17 != false) && {
                    bVar1 = true;
                    (*k).climbing_related_c == 0
                } {
                    bVar1 = false;
                }
                if (*k).climb_timer_possibly != 0 {
                    bVar1 = true;
                    (*k).climb_timer_possibly = (*k).climb_timer_possibly + -1;
                }
            }
            if ps2::game_mode() == 3 {
                ((*k).position).y = -(-400.0 - (*k).radius);
                x0zw.x = ((*k).dual_a).h.x;
                y = (*k).dual_a.h.y;
                x0zw.z = ((*k).dual_a).h.z;
                x0zw.w = ((*k).dual_a).h.w;
                ps2::x25fb0(&x0zw);
            } else if bVar1 {
                if (*k).currently_climbing == false {
                    // Branch 1, address eabb: happens normally
                    w = ((*k).position).w;
                    ((*k).position).x = vel.x + ((*k).position).x;
                    ((*k).position).y = vel.y + ((*k).position).y;
                    ((*k).position).z = vel.z + ((*k).position).z;
                    ((*k).position).w = vel.w + w;
                    ((*k).position).w = w;
                    // Disabling the code below this line seems to make it so that when you bump
                    // into something, the recoil lasts forever
                    x0zw.x = ((*k).dual_a).external_velocity.x + ((*k).position).x;
                    y = ((*k).position).y + ((*k).dual_a).external_velocity.y;
                    x0zw.z = ((*k).position).z + ((*k).dual_a).external_velocity.z;
                    x0zw.w = w + ((*k).dual_a).external_velocity.w;
                    ((*k).position).x = x0zw.x;
                    ((*k).position).y = y;
                    ((*k).position).z = x0zw.z;
                    ((*k).position).w = x0zw.w;
                    ((*k).position).w = w;
                } else {
                    if (*k).climb_height_reached == false {
                        // Branch 2, address ea4c: happens when climbing
                        w_ = ((*k).position).w;
                        x0zw.x = vel.x + ((*k).position).x;
                        y = vel.y + ((*k).position).y;
                        x0zw.z = vel.z + ((*k).position).z;
                        ((*k).position).x = x0zw.x;
                        ((*k).position).y = y;
                        ((*k).position).z = x0zw.z;
                        ((*k).position).w = vel.w + w_;
                        ((*k).position).w = w_;
                        x0zw.w = vel.w + w_;
                    }
                    ps2::climb_guy(output as _, k);
                }
            }
            if (*k).climbing_related_c != 0 {
                vel.x = ((*k).dual_a).external_velocity.x + ((*k).dual_a).f.x;
                vel.y = ((*k).dual_a).f.y + ((*k).dual_a).external_velocity.y;
                vel.z = ((*k).dual_a).f.z + ((*k).dual_a).external_velocity.z;
                velW = ((*k).dual_a).f.w + ((*k).dual_a).external_velocity.w;
                vel.w = velW;
            }
            speed = (vel.x * vel.x + vel.y * vel.y + vel.z * vel.z).sqrt();
            (*k).guy_that_gets_divided = speed;
            ps2::update_katamari_measurements(k);
            y = vel.y;
            x0zw.x = vel.x;
            x0zw.w = vel.w;
            x0zw.z = vel.z;
            ps2::katamari_physics_sub_sub1(k, &x0zw);
            ps2::x20660(k);
        } else {
            ps2::update_katamari_measurements(k);
            ps2::katamari_physics_big_subroutine(k);
        }
        /*
        lVar2 = 4;
        lVar3 = 4;
        do {
          lVar3 += -1;
        } while (lVar3 != 0);
        do {
          lVar2 += -1;
        } while (lVar2 != 0);
        */
        (*k).x434 = (*k).guy_that_gets_divided;
        x0zw.x = ((*k).position).x;
        y = (*k).position.y;
        x0zw.z = ((*k).position).z;
        x0zw.w = ((*k).position).w;
        ps2::apply_deadzones(&mut (*k).position, &x0zw, 0.001);
        speed = (*k).guy_that_gets_divided / (*k).x80;
        (*k).x88 = speed;
        (*k).x800 = 1.0 - speed * ps2::val_x7b218();
        ps2::gravity_user_3(k);
        (*k).x764.0 = 0;
        if ps2::multiplayer() == true {
            pIdx = (*k).player_index;
            pIdx_ = pIdx as u64;
            if CAMERAS[pIdx_ as usize].animation_mode == 5 {
                speed = (*k).radius * 0.5;
                (*k).xc2 = true;
                deltaX = ((*k).position_b).x - ((*k).position).x;
                deltaY = ((*k).position_b).y - ((*k).position).y;
                deltaZ = ((*k).position_b).z - ((*k).position).z;
                diffSpeed = (deltaX * deltaX + deltaY * deltaY + deltaZ * deltaZ).sqrt();
                if speed <= diffSpeed {
                    speed = diffSpeed;
                }
                (*k).timer_x3ad0 = (*k).timer_x3ad0 + 1;
                speed = (*k).x3ac8 - speed;
                (*k).x3ac8 = speed;
                if speed <= 0.0 {
                    if KATAMARI[pIdx_ as usize].xc2 != false {
                        KATAMARI[pIdx_ as usize].xc2 = false;
                        KATAMARI[pIdx_ as usize].x3ac8 = 0.0;
                        ps2::set_player_animation_mode(pIdx_ as i32, 6);
                        PRINCE[pIdx as usize].ouji_state.x19 = 0;
                        prince = &mut PRINCE[pIdx as usize];
                        ((*prince).ouji_state).x0 = 0;
                        ((*prince).ouji_state).x1 = 0;
                        ((*prince).ouji_state).freeze_counter = false;
                        ((*prince).ouji_state).x3 = 0;
                        PRINCE[pIdx as usize].x47c.0 = 0;
                        PRINCE[pIdx as usize].ouji_state.x16 = 0;
                        PRINCE[pIdx as usize].ouji_state.x17 = false;
                    }
                }
            } else {
                (*k).xc2 = false;
            }
        }
        if ps2::current_stage() == 3 {
            (*k).xc0 = 1200.0 <= (*k).diameter;
        }
        return;
    }
}
