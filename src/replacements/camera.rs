use super::*;

#[unsafe(naked)]
pub unsafe extern "C" fn orbit_a() {
    static THETA: f32 = 2.4_f32.to_radians();
    naked_asm! {
        "movss xmm0, [rip + {theta}]",
        "movss xmm1, [rdi + 0x964]",
        "vfmadd231ss xmm1, xmm0, [rip + {dt}]",
        "ret",
        theta = sym THETA,
        dt = sym values::DT_TICKS,
    }
}

#[unsafe(naked)]
pub unsafe extern "C" fn orbit_b() {
    static THETA: f32 = -1.2_f32.to_radians();
    static NEG_PI: f32 = -std::f32::consts::PI;
    naked_asm! {
        "movss xmm0, [rip + {theta}]",
        "movss xmm1, [rdi + r15 + 0xd3327c]",
        "vfmadd231ss xmm1, xmm0, [rip + {dt}]",
        "movss xmm0, [rip + {neg_pi}]",
        "ret",
        theta = sym THETA,
        dt = sym values::DT_TICKS,
        neg_pi = sym NEG_PI,
    }
}

#[unsafe(naked)]
pub unsafe extern "C" fn zoom_out() {
    naked_asm! {
        "movss xmm6, [rdi + 0x910]",            // xmm6 = c->distance
        "movss xmm0, [rdi + 0x914]",            // xmm0 = c->zoom_speed
        "vfmadd231ss xmm6, xmm0, [rip + {dt}]", // xmm6 = xmm0 * dt + xmm6
        "ret",
        dt = sym values::DT_TICKS,
    }
}
