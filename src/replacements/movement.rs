use super::*;

#[unsafe(naked)]
pub unsafe extern "C" fn airborne_gravity() {
    naked_asm! {
        // xmm5 is currently initialized to the katamari's f32@0x1a0 field,
        // which seems to be the gravitational acceleration value.
        "mulss xmm5, [rip + {dt}]",            // delta-time it
        "addss xmm5, dword ptr [rbx + 0x2e4]", // do the original thing: add to the previous velocity
        "movss xmm1, dword ptr [rbx + 0x2e8]", // trampoline to have enough room to fit the hook
        "ret",
        dt = sym values::DT_TICKS,
    }
}

pub unsafe extern "C" fn uphill() {
    let downhill_v: Vec4;
    let k: &mut Katamari;
    let p: &mut Prince;
    unsafe {
        let (x, y, z);
        asm! {
            "",
            out("xmm13") x,
            out("xmm14") y,
            out("xmm15") z,
        }
        downhill_v = Vec4::new(x, y, z, 0.0);
        k = ps2::current_katamari();
        p = ps2::current_prince();
    }

    k.time_spent_going_downhill = 0;
    // This is probably one of the most likely things to overflow.
    // Fortunately, it's completely fine to saturate in this case, I think.
    k.time_spent_going_uphill = k
        .time_spent_going_uphill
        .saturating_add(values::dt_millis() as u16);

    k.slope_state = 1;

    // A larger katamari is easier to push uphill. (This number decreases with size.)
    let mass_factor = if p.push_direction_z_ness <= 0.0 {
        k.big_uphill_mass_factor
    } else {
        // The force is diminished even further if the player is pushing forward or backward.
        k.small_uphil_mass_factor
    };

    let gdot = ps2::gravity().dot3(&k.ground_normal);
    let theta = if (-1.0..=1.0).contains(&gdot) {
        gdot.acos()
    } else {
        0.0
    };

    let dt = values::dt_ticks();

    // As you push the katamari uphill, it gradually gets more difficult.
    // This meter depletes faster for a steeper slope,
    // and replenishes instantly upon reaching level terrain.
    let divisor = 0.74 * PI / 2.0;
    let drain_amount = (theta / divisor).clamp(0.0, 1.0);
    p.slope_stamina -= drain_amount * p.slope_stamina_drain_rate * dt;
    p.slope_stamina = p.slope_stamina.max(0.0);

    let time_factor = (k.time_spent_going_uphill as f32 / 20.0) * (30.0 / 1000.0);
    let time_factor = time_factor.min(1.0);

    let amount = time_factor * (k.diameter_cm / 50.0) * mass_factor * dt;
    k.motion.downhill += downhill_v * amount;
}

pub unsafe extern "C" fn downhill() {
    let downhill_v: Vec4;
    let k: &mut Katamari;
    unsafe {
        let (x, y, z);
        asm! {
            "",
            out("xmm13") x,
            out("xmm14") y,
            out("xmm15") z,
        }
        downhill_v = Vec4::new(x, y, z, 0.0);
        k = ps2::current_katamari();
    }

    k.time_spent_going_uphill = 0;
    k.time_spent_going_downhill = k
        .time_spent_going_downhill
        .saturating_add(values::dt_millis() as u16);

    k.slope_state = 2;

    let time_factor = (k.time_spent_going_downhill as f32 / 20.0) * (30.0 / 1000.0);
    let time_factor = time_factor.min(1.0);

    let amount = time_factor * (k.diameter_cm / 50.0) * values::dt_ticks();
    k.motion.downhill += downhill_v * amount;
}

#[unsafe(naked)]
pub unsafe extern "C" fn bumpy_ride() {
    naked_asm! {
        "mulss xmm6, [rip + {dt}]", // the important bit
        "movss dword ptr [rdi + 0x39b8], xmm6",
        // trampoline. extra 8 bytes for this function's return pointer
        "lea rdx, [rsp + 0x28]",
        "ret",
        dt = sym values::DT_TICKS,
    }
}

#[unsafe(naked)]
pub unsafe extern "C" fn normal_motion_branch_first_part() {
    naked_asm! {
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
        "ret",

        dt = sym values::DT_TICKS,
    }
}

pub unsafe extern "C" fn standing_on_prop_branch() {
    unsafe {
        let k: *mut Katamari;
        asm!("mov rcx, rbx", out("rcx") k);
        ps2::update_katamari_measurements(k);
        ps2::katamari_pivot(k);
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

// Turning in place with both sticks in opposite directions.
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

// Turning in place with one stick neutral and one stick forward or backward.
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

// Steering while moving.
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

#[unsafe(naked)]
pub unsafe extern "C" fn spin_amount() {
    naked_asm! {
        "movss dword ptr [rsp + 0x2c + 8], xmm9",
        // this feels like a hack - angular speed should use the same measurement no matter which state you're in
        "mov al, byte ptr [rdi + 0xa4]", // al = k->airborne
        "cmp al, 0",
        "je not_airborne",
        "mulss xmm2, dword ptr [rip + {dt}]",
        "not_airborne:",
        "jmp {}",
        sym ps2::rotation_from_axis_angle,
        dt = sym values::DT_TICKS,
    }
}
