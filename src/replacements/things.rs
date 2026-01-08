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
