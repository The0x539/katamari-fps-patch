use super::*;

static mut RUMBLE_BIAS: [f32; 2] = [0.0; 2];

#[unsafe(export_name = "ResetRumbleBias")]
unsafe extern "C" fn reset_rumble_bias() {
    unsafe { RUMBLE_BIAS = [0.0; 2] }
}

#[unsafe(export_name = "GetRumbleBias")]
unsafe extern "C" fn get_rumble_bias(p_idx: usize) -> f32 {
    unsafe { RUMBLE_BIAS[p_idx] }
}

fn on_bump(katamari: &Katamari, direction: Vec2) {
    if direction.sqrlen() < 0.1 {
        return;
    }

    // Consider a view of the katamari and the bumped thing, from above.
    // Consider a unit circle drawn around the center of the katamari.
    // Consider the thing's position (relative to the katamari), projected onto that circle.
    // This value should be the X coordinate of that projected point.
    //
    // In other words, this value is:
    // - -1.0 if the thing is directly to the katamari's left
    // - +1.0 if the thing is directly to the katamari's right
    // - ±0.0 if the thing is directly to the katamari's front or back
    let cos_theta = katamari.left_direction.xz().norm().dot(direction.norm());

    unsafe {
        RUMBLE_BIAS[ps2::current_player_index() as usize] = cos_theta;
    }
}

pub unsafe extern "C" fn pickup_hook() {
    let (thing, katamari) = unsafe {
        let t_ptr: *const Thing;
        let k_ptr: *const Katamari;
        asm! {
            "mov [rdi + 0xa88], r8w", // trampoline
            out("rdi") t_ptr,
            out("r14") k_ptr,
        };

        (&*t_ptr, &*k_ptr)
    };

    let displacement = katamari.position.xz() - thing.position.xz();
    on_bump(katamari, displacement);
}

pub unsafe extern "C" fn wall_bump_hook() {
    let katamari = unsafe {
        let k_ptr: *const Katamari;
        asm! {
            "mov {}, rbx",
            out(reg) k_ptr,
        }
        &*k_ptr
    };

    on_bump(katamari, katamari.wall_normal.xz());

    unsafe {
        asm! {
            "mov rbx, {}",
            "movups xmm0, [rbx + 0x79c]", // trampoline
            in(reg) katamari,
        }
    }
}

// The trampoline instruction needs to go in a function with a known stack frame size.
#[unsafe(naked)]
pub unsafe extern "C" fn thing_bump_hook() {
    naked_asm! {
        "mov [rsp + 0x18 + 8], rbx", // trampoline
        "jmp {}",
        sym thing_bump_hook_impl,
    }
}

unsafe extern "C" fn thing_bump_hook_impl(katamari: &Katamari, thing: &Thing) {
    let displacement = katamari.position.xz() - thing.position.xz();
    on_bump(katamari, displacement);

    unsafe { asm! ("", in("rcx") katamari, in("rdx") thing) }
}
