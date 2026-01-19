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

#[unsafe(naked)]
pub unsafe extern "C" fn hop_timer_reset() {
    naked_asm! {
        // Praying that the byte after this counter is unused,
        // at least by the NPCs we care about.
        "mov rax, [rsp + 8]",
        "mov word ptr [rax + 0x90], 833", // 25 ticks -> 833 ms
        "mov word ptr [rbx + 0x58e], 1",
        "ret",
    }
}

#[unsafe(naked)]
pub unsafe extern "C" fn hop_timer_update() {
    naked_asm! {
        "mov ax, [rcx + 0x90]",
        "cmp ax, 0",
        "jle 2f",

        "sub ax, [rip + {dt}]",
        "jns 3f",
        "xor eax, eax",
        "3:",
        "mov [rcx + 0x90], ax",

        // the return of the cursed double return
        "add rsp, 0x28",
        "pop rbx",
        "ret",

        "2:",
        "ret",

        dt = sym values::DT_MILLIS,
    }
}

#[unsafe(naked)]
pub unsafe extern "C" fn hop_angle_update() {
    naked_asm! {
        "movss xmm2, [rdi + 0x78]",
        "movss xmm1, [rdi + 0x74]",
        "vfmadd231ss xmm1, xmm0, [rip + {dt}]",
        "ret",
        dt = sym values::DT_TICKS,
    }
}

#[unsafe(naked)]
pub unsafe extern "C" fn hop_position_update() {
    naked_asm! {
        "movss xmm15, [rcx + 0x9c]",
        "movups xmm3, [rcx + 0x90]",
        "vbroadcastss xmm1, [rip + {dt}]",
        "vfmadd231ps xmm3, xmm1, [rcx + 0xe0]",
        "movups [rcx + 0x90], xmm3",
        "movss [rcx + 0x9c], xmm15",
        "ret",
        dt = sym values::DT_TICKS,
    }
}

#[unsafe(naked)]
pub unsafe extern "C" fn hop_gravity() {
    naked_asm! {
        "movss xmm3, [rdi + 0x88]",             // xmm3 = t->hop_grav_vel
        "vfmadd132ss xmm0, xmm3, [rip + {dt}]", // xmm0 = (xmm0 * dt) + xmm3
        "mulss xmm2, [rdi + 0x28]",
        "movaps xmm3, xmm1",
        "ret",
        dt = sym values::DT_TICKS,
    }
}

pub unsafe extern "C" fn sink_rate() {
    unsafe {
        let k: *mut Katamari;
        asm!("", out("rdx") k);

        (*k).sink_rate = (*k).sink_rate.powf(values::DT_TICKS);

        // RCX and RAX would theoretically also be good to preserve,
        // but I think only RDX is strictly necessary based on the caller's register usage.
        asm!("", in("rdx") k);
    }
}
