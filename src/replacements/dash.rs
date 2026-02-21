use super::*;

#[link(name = "native_replacements", kind = "static")]
unsafe extern "C" {
    pub fn stamina_gain();
    pub fn stamina_drain();
    pub fn update_dash_input_timer();
    pub fn prince_exhausted();
    pub fn multiplayer_dash_input_window();
    pub fn multiplayer_dash_deplete_distance();
    pub fn update_spindown_timer();
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
