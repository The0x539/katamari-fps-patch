use std::arch::asm;
use std::f32::consts::{PI, TAU};

use crate::ps2;
use crate::types::{Camera, Katamari, Mat4, Prince, Vec4};

pub(crate) mod values {
    /// Let a "tick" refer to a 1/30 second duration.
    /// Let an "update" refer to one call of the Tick() function from PS2KatamariSimulation.dll.
    ///
    /// The vanilla game assumes that each update simulates one tick.
    /// Changing this requires a great deal of x86 surgery.
    ///
    /// Mutable variables in this module will be updated once each tick.
    /// Doing this ahead of time, and having these variables within this DLL's address space,
    /// allows the hooks' inline assembly to use the values with fewer instructions and fewer registers.
    ///
    /// (The initial values are not expected to be observed, but are based on having 60 updates per second.)

    /// The duration, in *seconds*, of the current update.
    ///
    /// Corresponds directly to Unity's `time.deltaTime`,
    /// or at least will when the C# side of this mod is complete.
    pub static mut DT_SECONDS: f32 = 1.0 / 60.0;

    /// The duration, in *ticks*, of the current update.
    ///
    /// If a value is measured in "units per tick", e.g. velocity,
    /// it should probably be multiplied by this value.
    pub static mut DT_TICKS: f32 = 0.5;

    /// The duration, in (rounded) *milliseconds*, of the current update.
    ///
    /// If the original code uses an integer to count ticks,
    /// then updating it to instead count milliseconds
    /// will require using this value (instead of 1) as an increment/decrement.
    pub static mut DT_MILLIS: i16 = 16;

    /// The accumulated rounding error of DT_MILLIS.
    static mut DT_MICROS: i16 = 0;

    pub fn set_dt(delta: f32) {
        unsafe {
            DT_SECONDS = delta;
            DT_TICKS = delta * 1000.0 / 30.0;

            let millis = delta * 1000.0;
            DT_MILLIS = millis.floor() as i16;

            let micros = (millis - millis.floor()) * 1000.0;
            DT_MICROS += micros.round() as i16;
            while DT_MICROS > 1000 {
                DT_MILLIS += 1;
                DT_MICROS -= 1000;
            }
        }
    }

    /// The duration, in (rounded) milliseconds, of *one tick*.
    ///
    /// If the original code uses an integer to count ticks,
    /// then updating it to instead count milliseconds
    /// will require multiplying the maximum (*i.e.*: final if counting up; initial if counting down) value
    /// by this ratio, preferably via that timer's "reset_value" field during initialization,
    /// or by changing the hardcoded constant that the timer is compared to.
    ///
    /// To reduce rounding errors, prefer *not* to actually use this constant,
    /// instead using the (n * 1000) / 30 order of operations by doing the math inline.
    #[allow(dead_code)]
    pub const TICK_MS: i16 = 1000 / 30;

    pub static mut MULTIPLAYER: u8 = 0;
    // TODO: add more stuff here as necessary and update it whenever needed

    // Exposed for convenience from safe-Rust code.
    #[inline]
    pub(super) fn dt_millis() -> i16 {
        unsafe { DT_MILLIS }
    }
}

pub unsafe extern "C" fn normal_motion_branch_first_part() {
    unsafe {
        asm! {
            "mulss xmm6, dword ptr [rip+{dt}]",
            "mulss xmm7, dword ptr [rip+{dt}]",
            "mulss xmm8, dword ptr [rip+{dt}]",

            // the original code
            "movss xmm4, dword ptr [rbx+0x46C]",
            "movaps xmm3, xmm6",
            "addss xmm3, dword ptr [rbx+0x460]",
            "movaps xmm0, xmm7",
            "movaps xmm1, xmm8",
            "addss xmm0, dword ptr [rbx+0x464]",
            "addss xmm1, dword ptr [rbx+0x468]",

            dt = sym values::DT_TICKS,
        }
    }
}

pub unsafe extern "C" fn standing_on_prop_branch() {
    unsafe {
        let k: *mut Katamari;
        asm!("mov rcx, rbx", out("rcx") k);
        ps2::update_katamari_measurements(k);
        ps2::katamari_physics_prop_collision(k);
    }
}

pub unsafe extern "C" fn prop_terrain_collision() {
    unsafe {
        asm! {
            // the original code, optimized:
            // calculate the length of k->dual_a.f and store it in xmm0
            // (to then be stored in k.guy_that_gets_divided)
            "movaps xmm0, xmmword ptr [rdi+0x290]", // v = k->dual_a.f
            "mulps xmm0, xmm1",  // v = v.xyz()
            "mulps xmm0, xmm0",  // v *= v
            "haddps xmm0, xmm0", // v = v.xzxz + v.ywyw (first element is now x*x+y*y, second element is z*z)
            "haddps xmm0, xmm0", // v = v.xzxz + v.ywyw (first element is now sqrlen)
            "sqrtss xmm0, xmm0", // v.x = sqrt(v.x)

            "mulss xmm0, dword ptr [rip+{dt}]", // now delta-time it

            in("xmm1") Vec4::XYZ.to_simd(),
            dt = sym values::DT_TICKS,
        }
    }
}

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

pub unsafe extern "C" fn fast_steer() {
    unsafe {
        let prince: *mut Prince;
        let direction: f32;
        asm!("", out("rcx") prince, out("xmm1") direction);

        asm! {
            "movss dword ptr [rcx+0x78], xmm1",
            "mulss xmm1, dword ptr [rip+{dt}]",
            "mulss xmm1, {gkr}",
            "addss xmm1, dword ptr [rcx+0x6c]",

            in("xmm1") direction,
            in("rcx") prince,
            dt = sym values::DT_TICKS,
            gkr = in(xmm_reg) ps2::g_katamari_rot(),
        }
    }
}

pub unsafe extern "C" fn slow_steer() {
    unsafe {
        let prince: *mut Prince;
        let direction: f32;
        asm! {
            "push rbx",
            "mov {}, rbx",
            out(reg) prince,
            out("xmm1") direction,
        };

        asm! {
            "movss dword ptr [{prince}+0x78], xmm1",
            "mulss xmm1, xmm0",
            "mulss xmm1, dword ptr [rip+{dt}]",
            "addss xmm1, dword ptr [{prince}+0x6c]",
            "pop rbx",

            in("xmm0") ps2::g_katamari_rot(),
            in("xmm1") direction,
            dt = sym values::DT_TICKS,
            prince = in(reg) prince,
        }
    }
}

pub unsafe extern "C" fn gentle_steer() {
    unsafe {
        let prince: *mut Prince;
        asm!("mov {}, rbx", out(reg) prince);

        asm! {
            "mulss xmm1, dword ptr [rip+{dt}]",
            "mulss xmm1, dword ptr [{prince}+0x78]",
            in("xmm1") ps2::g_katamari_rot(),
            dt = sym values::DT_TICKS,
            prince = in(reg) prince,
        }
    }
}

pub unsafe extern "C" fn sfx_npc_approaching() {
    unsafe {
        let i = ps2::current_player_index();
        let k = ps2::katamari_array(i);

        (*k).sfx_0x2d_timer -= (ps2::delta_time() * 1000.0) as i16;
        // println!("{}", (*k).sfx_0x2d_timer);
        if (*k).sfx_0x2d_timer < 0 {
            // TODO: these two values remain mysterious
            if ps2::val_x10daed() && ps2::val_x0ff0f6() == 0 {
                ps2::cb::play_sound_fx(0x2d, 1.0, 0);
            }

            (*k).sfx_0x2d_timer = (*k).sfx_0x2d_interval * 1000 / 30;
        }
    }
}

// Shelved for now. Harder to do than I expected.
pub unsafe extern "C" fn prince_flip() {
    unsafe {
        let prince: *mut Prince;
        asm!("", out("rdi") prince);

        let mut yaw = (*prince).actual_yaw;
        if yaw.is_nan() {
            yaw = 0.0;
        }
        // TODO: I'm not sure this is actually ever used properly,
        // but it might have something to do with the joysticks
        let foo = (*prince).prince_flip_rate_guy;
        if foo != 0.0 {
            println!("{foo}");
        }
        yaw += foo * values::DT_TICKS;
        if yaw > PI {
            yaw -= TAU;
        }
        if yaw < -PI {
            yaw += TAU;
        }
        (*prince).actual_yaw = yaw;

        // this seems to be the only part of register state
        // that the function touches in the replaced block and also uses later
        asm! {
            "xor rbx, rbx",
            "mov rcx, rbx",
        }
    }
}

pub unsafe extern "C" fn stamina_gain() {
    let prince = unsafe {
        let prince: *mut Prince;
        asm!("mov rcx, rbx", out("rcx") prince);
        &mut *prince
    };

    if prince.ouji_state.dash_spinning {
        prince.stamina_gain_timer = 0;
        return;
    }

    prince.stamina_gain_timer += values::dt_millis();

    if prince.stamina_gain_timer as i32 > prince.stamina_gain_interval {
        prince.stamina_gain_timer = 0;

        let mut stamina = prince.stamina as i32;
        stamina += prince.stamina_gain_amount;
        stamina = stamina.min(prince.stamina_limit);
        prince.stamina = stamina as i16;
    }
}

pub unsafe extern "C" fn stamina_drain() {
    unsafe {
        asm! {
            "mov ax, word ptr [rdi + 0x47e]",
            "sub ax, word ptr [rip + {dt}]",
            "mov word ptr [rdi + 0x47e], ax",
            "cmp ax, bp",
            dt = sym values::DT_MILLIS,
        }
    }
}

pub unsafe extern "C" fn update_dash_input_timer() {
    unsafe {
        asm! {
            "mov ax, word ptr [rdi + 0x478]", // ax = prince->dash_input_timer
            "sub ax, word ptr [rip + {dt}]",  // ax -= DT_MILLIS
            "jns 2f",                         // if ax < 0 {
            "xor eax, eax",                   //     eax = 0
            "2:",                             // }
            "mov word ptr [rdi + 0x478], ax", // prince->dash_input_timer = ax
            "cmp byte ptr [rip + {mp}], sil", // annoying trampoline
            dt = sym values::DT_MILLIS,
            mp = sym values::MULTIPLAYER,
        }
    }
}

pub unsafe extern "C" fn prince_exhausted(p_idx: i32, prince: *mut Prince) {
    let (prince, katamari) = unsafe { (&mut *prince, &mut *ps2::katamari_array(p_idx)) };

    if !ps2::multiplayer() {
        prince.ouji_state.dash_pending = false;
        prince.ouji_state.dash_x1 = 0;
        prince.ouji_state.dash_spinning = false;
        prince.ouji_state.dash_stationary_spin = false;
        prince.dash_input_counter = 0;
        katamari._xba.0 = 0;
    }

    if prince.prevent_dashing {
        prince.exhaustion_timer -= values::dt_millis();
        if prince.exhaustion_timer <= 0 {
            ps2::prince_reset_exhaustion(prince);
        }
    }
}

pub unsafe extern "C" fn dash_state_machine() {
    let (prince, k) = unsafe {
        let prince: *mut Prince;
        let k: *mut Katamari;
        asm!("", out("rsi") prince, out("r14") k);
        (&mut *prince, &mut *k)
    };

    match k.dash_state {
        0 => {
            if !k.hit_water || k.dash_timer != 0 {
                prince.ouji_state.dash_active = true;
                k.dash_timer += values::dt_millis();
                if k.dash_timer >= 500 {
                    k.dash_state = 1;
                }
                if k.hit_water {
                    k.dash_state = 2;
                    k.dash_timer = 333;
                }
            } else {
                k.dash_state = 3;
            }
        }
        1 => {
            prince.ouji_state.dash_active = true;
            if k.dwordflag_xa7[0] != 0 || !prince.ouji_state.dash_stationary_spin {
                k.dash_state = 2;
                k.dash_timer = 500;
            }
            if k.hit_water {
                k.dash_state = 2;
                k.dash_timer = 333;
            }
        }
        2 => {
            prince.ouji_state.dash_active = false;
            k.dash_timer -= values::dt_millis();
            if k.dash_timer <= 0 {
                k.dash_state = 3;
            }
        }
        3 => prince.ouji_state.dash_active = false,
        _ => {}
    }
}

pub unsafe extern "C" fn prince_post_init() {
    for i in 0..=1 {
        let prince = unsafe { &mut *ps2::prince_array(i) };

        // Timer initial/maximum values. Originally counted ticks; will now count milliseconds.
        for val in [
            &mut prince.max_exhaustion,
            &mut prince.stamina_gain_amount,
            &mut prince.stamina_gain_interval,
            &mut prince.stamina_limit,
            &mut prince.dash_input_window,
        ] {
            *val = (*val * 1000) / 30;
        }

        prince.stamina = prince.stamina_limit as i16;

        unsafe {
            values::MULTIPLAYER = ps2::multiplayer() as u8;
        }
    }
}

pub unsafe extern "C" fn copy_matrix(dst: *mut Mat4, src: *const Mat4) -> *mut Mat4 {
    unsafe {
        // compiles to two ymmword load/store pairs
        // TODO: make sure clobbering ymm0/ymm1 is okay for the caller...
        *dst = *src;
        dst
    }
}

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
                if (*ps2::prince_array((*k).player_index)).ouji_state.x17 {
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
            ps2::katamari_physics_prop_collision(k);
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
            if (*ps2::camera_array(p_idx)).animation_mode == 5 {
                let mut n = (*k).radius * 0.5;
                (*k).xc2 = true;
                let delta = (*k).position_b - (*k).position;
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
                        ps2::set_player_animation_mode(p_idx as i32, 6);
                        let prince = &mut *ps2::prince_array(p_idx);
                        prince.ouji_state.x19 = 0;
                        prince.ouji_state.dash_pending = false;
                        prince.ouji_state.dash_x1 = 0;
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

        let PRINCE: &mut [Prince; 2] = &mut *ps2::prince_array(0).cast();
        let CAMERAS: &mut [Camera; 2] = &mut *ps2::camera_array(0).cast();
        let KATAMARI: &mut [Katamari; 2] = &mut *ps2::katamari_array(0).cast();

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
            ps2::katamari_physics_prop_collision(k);
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
                        ((*prince).ouji_state).dash_pending = false;
                        ((*prince).ouji_state).dash_x1 = 0;
                        ((*prince).ouji_state).dash_spinning = false;
                        ((*prince).ouji_state).dash_stationary_spin = false;
                        PRINCE[pIdx as usize].dash_input_counter = 0;
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
