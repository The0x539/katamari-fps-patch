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

#[unsafe(naked)]
pub unsafe extern "C" fn spindash_gain_power() {
    naked_asm! {
        "movss xmm2, [rbx + 0x420]",
        "movss xmm0, [rbx + 0x3a7c]",
        "vfmadd132ss xmm2, xmm0, [rip + {dt}]",
        "ret",
        dt = sym values::DT_TICKS,
    }
}

#[unsafe(naked)]
pub unsafe extern "C" fn spindash_spinning() {
    naked_asm! {
        "lea rcx, [rbx + 0x3a84]",
        "lea rdx, [rsp + 0x58 + 8]",
        "movups [rsp + 0x58 + 8], xmm0",
        // all of that was just a trampoline lmao
        "mulss xmm2, [rip + {dt}]",
        "ret",
        dt = sym values::DT_TICKS,
    }
}
