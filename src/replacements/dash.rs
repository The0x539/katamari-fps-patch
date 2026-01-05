use super::*;

pub unsafe extern "C" fn stamina_gain() {
    let prince = unsafe {
        let prince: *mut Prince;
        asm!("mov rcx, rbx", out("rcx") prince);
        &mut *prince
    };

    if prince.ouji_state.dash_spinning {
        prince.stamina_gain_timer = 0;
        return;
    }

    prince.stamina_gain_timer += values::dt_millis();

    if prince.stamina_gain_timer as i32 > prince.stamina_gain_interval {
        prince.stamina_gain_timer = 0;

        let mut stamina = prince.stamina as i32;
        stamina += prince.stamina_gain_amount;
        stamina = stamina.min(prince.stamina_limit);
        prince.stamina = stamina as i16;
    }
}

pub unsafe extern "C" fn stamina_drain() {
    unsafe {
        asm! {
            "mov ax, word ptr [rdi + 0x47e]",
            "sub ax, word ptr [rip + {dt}]",
            "mov word ptr [rdi + 0x47e], ax",
            "cmp ax, bp",
            dt = sym values::DT_MILLIS,
        }
    }
}

pub unsafe extern "C" fn update_dash_input_timer() {
    unsafe {
        asm! {
            "mov ax, word ptr [rdi + 0x478]", // ax = prince->dash_input_timer
            "sub ax, word ptr [rip + {dt}]",  // ax -= DT_MILLIS
            "jns 2f",                         // if ax < 0 {
            "xor eax, eax",                   //     eax = 0
            "2:",                             // }
            "mov word ptr [rdi + 0x478], ax", // prince->dash_input_timer = ax
            "cmp byte ptr [rip + {mp}], sil", // annoying trampoline
            dt = sym values::DT_MILLIS,
            mp = sym values::MULTIPLAYER,
        }
    }
}

pub unsafe extern "C" fn prince_exhausted(p_idx: i32, prince: *mut Prince) {
    let (prince, katamari) = unsafe { (&mut *prince, &mut *ps2::katamari_array(p_idx)) };

    if !ps2::multiplayer() {
        prince.ouji_state.dash_pending = false;
        prince.ouji_state.dash_base_requirement_met = false;
        prince.ouji_state.dash_spinning = false;
        prince.ouji_state.dash_stationary_spin = false;
        prince.dash_input_counter = 0;
        katamari.versus_xba = false;
    }

    if prince.prevent_dashing {
        prince.exhaustion_timer -= values::dt_millis();
        if prince.exhaustion_timer <= 0 {
            ps2::prince_reset_exhaustion(prince);
        }
    }
}

pub unsafe extern "C" fn dash_state_machine() {
    let (prince, k) = unsafe {
        let prince: *mut Prince;
        let k: *mut Katamari;
        asm!("", out("rsi") prince, out("r14") k);
        (&mut *prince, &mut *k)
    };

    match k.dash_state {
        0 => {
            if !k.hit_water || k.dash_timer != 0 {
                prince.ouji_state.dash_active = true;
                k.dash_timer += values::dt_millis();
                if k.dash_timer >= 500 {
                    k.dash_state = 1;
                }
                if k.hit_water {
                    k.dash_state = 2;
                    k.dash_timer = 333;
                }
            } else {
                k.dash_state = 3;
            }
        }
        1 => {
            prince.ouji_state.dash_active = true;
            if k.dwordflag_xa7[0] != 0 || !prince.ouji_state.dash_stationary_spin {
                k.dash_state = 2;
                k.dash_timer = 500;
            }
            if k.hit_water {
                k.dash_state = 2;
                k.dash_timer = 333;
            }
        }
        2 => {
            prince.ouji_state.dash_active = false;
            k.dash_timer -= values::dt_millis();
            if k.dash_timer <= 0 {
                k.dash_state = 3;
            }
        }
        3 => prince.ouji_state.dash_active = false,
        _ => {}
    }
}
