use std::simd::f32x4;

use super::*;

#[unsafe(naked)]
pub unsafe extern "C" fn flip_duration() {
    static TICK_MS: f32 = 1000.0 / 30.0;
    naked_asm! {
        "addss xmm2, [rdx]",
        "mulss xmm2, [rip + {dt}]",
        "cvttss2si eax, xmm2",
        "mov [rbx + 0x2d0], eax",
        "ret",
        dt = sym TICK_MS,
    }
}

#[unsafe(naked)]
pub unsafe extern "C" fn flip_timer() {
    static W: f32x4 = Vec4::W.to_simd();
    naked_asm! {
        "mov edi, [rip + {dt}]",
        "sub [rcx + 0x44c], edi",
        "xorps xmm11, xmm11",
        "movaps xmm0, [rip + {w}]",
        "ret",
        dt = sym values::DT_MILLIS,
        w = sym W,
    }
}
