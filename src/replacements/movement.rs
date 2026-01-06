use super::*;

#[unsafe(naked)]
pub unsafe extern "C" fn gravity_mark3() {
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
