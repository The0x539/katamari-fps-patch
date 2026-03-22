use std::time::{Duration, Instant};

use super::*;

#[link(name = "native_replacements", kind = "static")]
unsafe extern "C" {
    pub fn bang2();
    pub fn brake_dust_timer_update();
    pub fn bump_scream_cooldown_update();
    pub fn big_dust_timer();

    pub fn splash_timer_a_update();
    pub fn splash_timer_b_update();
    pub fn splash_timer_c_update();
}

pub unsafe extern "C" fn sfx_npc_approaching() {
    unsafe {
        let i = ps2::current_player_index();
        let k = ps2::katamari_array(i);

        (*k).sfx_0x2d_timer -= values::dt_millis();
        if (*k).sfx_0x2d_timer < 0 {
            if ps2::allow_sfx() && ps2::val_x0ff0f6() == 0 {
                ps2::cb::play_sound_fx(0x2d, 1.0, 0);
            }

            (*k).sfx_0x2d_timer = (*k).sfx_0x2d_interval * 1000 / 30;
        }
    }
}

pub(super) static mut GUNSHOT_TIMESTAMP: Option<Instant> = None;

pub unsafe extern "C" fn bang() -> u32 {
    let too_soon =
        unsafe { GUNSHOT_TIMESTAMP }.is_some_and(|t| t.elapsed() < Duration::from_millis(200));

    if !too_soon {
        unsafe {
            GUNSHOT_TIMESTAMP = Some(Instant::now());
        }
    }

    // The original calling code computes a value for EAX by subtracting
    // its "now" timestamp from its "most recent gunshot" timestamp.
    // After this function returns, the calling code compares EAX to 0x6.
    // If EAX >= 6, then the block to play the SFX executes.
    // If EAX < 6, then the JC instruction branches, and the block is skipped.
    // The decompiled C code writes this comparison as `if (5 < foo) {`.
    // To minimize confusion, let's have this function return a value either smaller than 5 or larger than 6.
    if too_soon { 4 } else { 7 }

    // The volume of the sound effect is calculated based on two variables:
    // - Distance between NPC and katamari
    // - Diameter of katamari
    // The volume ramps linearly from maximum (1.0) to silent (0.0) as distance varies from 3 diameters to 12 diameters.
    // This has annoying and undesirable results when you reach 10+ meters while missing a policeman, but whatever.
    // Fixing every single issue in the game isn't my responsibility.
    // That said, if a good idea for how to address this unobtrusively comes across me, I might implement something.
}
