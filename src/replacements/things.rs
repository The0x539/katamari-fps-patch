use super::*;

#[unsafe(naked)]
pub unsafe extern "C" fn melon_spin() {
    static THETA: f32 = 0.15;
    naked_asm! {
        "movss xmm0, dword ptr [rip + {theta}]",
        "mulss xmm0, dword ptr [rip + {dt}]",
        "addss xmm0, dword ptr [rsi + 0x110]",
        "ret",
        theta = sym THETA,
        dt = sym values::DT_TICKS,
    }
}

#[unsafe(naked)]
pub unsafe extern "C" fn animal_walk() {
    naked_asm! {
        // bit annoying that this went in the *middle* of the replaced code
        "movups xmmword ptr [rbp - 0x79], xmm3",

        // I would love to make these properly SIMD,
        // but there's some really complicated swizzling right after this,
        // involving subtracting 5*size from the y component,
        // and then passing the vectors, using stack pointers, to some function.
        "movss xmm3, dword ptr [rbx + 0xe0]",
        "movss xmm0, dword ptr [rbx + 0xe4]",
        "movss xmm1, dword ptr [rbx + 0xe8]",
        "movss xmm4, dword ptr [rbx + 0xec]",

        "movaps xmm2, xmm4", // might need to swap these two registers

        "mulss xmm3, dword ptr [rip + {dt}]",
        "mulss xmm0, dword ptr [rip + {dt}]",
        "mulss xmm1, dword ptr [rip + {dt}]",
        "mulss xmm2, dword ptr [rip + {dt}]", // probably not strictly necessary

        "addss xmm3, dword ptr [rbx + 0x90]",
        "addss xmm0, dword ptr [rbx + 0x94]",
        "addss xmm1, dword ptr [rbx + 0x98]",
        "addss xmm2, dword ptr [rbx + 0x9c]", // what are we doing here?

        "ret",

        dt = sym values::DT_TICKS,
    }
}
