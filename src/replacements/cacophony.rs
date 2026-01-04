use super::*;

pub unsafe extern "C" fn sfx_npc_approaching() {
    unsafe {
        let i = ps2::current_player_index();
        let k = ps2::katamari_array(i);

        (*k).sfx_0x2d_timer -= (ps2::delta_time() * 1000.0) as i16;
        // println!("{}", (*k).sfx_0x2d_timer);
        if (*k).sfx_0x2d_timer < 0 {
            // TODO: these two values remain mysterious
            if ps2::val_x10daed() && ps2::val_x0ff0f6() == 0 {
                ps2::cb::play_sound_fx(0x2d, 1.0, 0);
            }

            (*k).sfx_0x2d_timer = (*k).sfx_0x2d_interval * 1000 / 30;
        }
    }
}

pub unsafe extern "C" fn splash() {
    let p_idx = ps2::current_player_index();
    let k = unsafe { &mut *ps2::katamari_array(p_idx) };

    if !k.hit_water {
        // TODO: I don't think I see anything when this happens.
        // Is there a bug in the original game?
        // Is this using the wrong diameter field, making it the wrong size?
        ps2::play_visual_fx(
            0x18,
            k.impact_point / 100.0,
            Vec4::ZERO,
            k.diameter_cm / 100.0,
            -1,
            p_idx as i32,
        );
    }

    k.hit_water = true;
    let dt = values::dt_millis();

    k.water_ripple_timer -= dt;
    if k.water_ripple_timer <= 0 {
        k.water_ripple_timer = 8 * 1000 / 30;

        // The return values from GetImpactNormal don't seem to be used from what I can tell.
        // The XMM registers used in this call are just populated with the same impact_point values.

        // Ripples
        ps2::play_visual_fx(
            0x11,
            k.impact_point / 100.0,
            Vec4::ZERO,
            k.diameter_m / 100.0,
            -1,
            0, // why not player index? I guess it doesn't matter if multiplayer has no water
        );
    }

    k.splash_sfx_timer -= dt;
    if k.splash_sfx_timer <= 0 {
        k.splash_sfx_timer = 24 * 1000 / 30;
        ps2::cb::play_sound_fx(0x30, 1.0, 0);
    }

    if !k.vfx_x17_flag && k.camera_mode_thing == 0 {
        k.water_droplet_timer -= dt;
        if k.water_droplet_timer <= 0 {
            k.water_droplet_timer = 1000 / 30; // One tick?! What is this?

            if !values::multiplayer() && k.x88 >= 0.05 {
                // Droplets
                ps2::play_visual_fx(
                    0x17,
                    k.impact_point / 100.0,
                    Vec4::ZERO,
                    k.diameter_m,
                    -1,
                    p_idx as i32,
                );
            }
        }
    }
}
