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

#[unsafe(naked)]
pub unsafe extern "C" fn katamari_view_ascend() {
    naked_asm! {
        "mov eax, ebx",
        "add eax, [rip + {dt}]",
        "movaps xmm6, xmm0",
        "mov [rsi + 0x884], eax",
        "ret",
        dt = sym values::DT_MILLIS,
    }
}

#[unsafe(naked)]
pub unsafe extern "C" fn katamari_view_descend() {
    naked_asm! {
        "mov edi, [rsi + 0x884]",
        "mov ebx, [rsi + 0x888]",
        "add edi, [rip + {dt}]",
        "ret",
        dt = sym values::DT_MILLIS,
    }
}
