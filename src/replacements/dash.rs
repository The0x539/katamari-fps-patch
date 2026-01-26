use super::*;

#[link(name = "native_replacements", kind = "static")]
unsafe extern "C" {
    pub fn stamina_gain();
    pub fn stamina_drain();
    pub fn update_dash_input_timer();
}

pub unsafe extern "C" fn prince_exhausted(p_idx: i32, prince: *mut Prince) {
    let (prince, katamari) = unsafe { (&mut *prince, &mut *ps2::katamari_array(p_idx)) };

    if !ps2::multiplayer() {
        prince.ouji_state.dash_pending = false;
        prince.ouji_state.dash_base_requirement_met = false;
        prince.ouji_state.dash_spinning = false;
        prince.ouji_state.dash_stationary_spin = false;
        prince.dash_input_counter = 0;
        katamari.spinning_in_place = false;
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
