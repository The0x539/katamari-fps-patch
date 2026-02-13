#![feature(slice_from_ptr_range)]
#![feature(portable_simd)]
#![feature(slice_ptr_get)]
// the big macro invocation for the patches uses a token-tree muncher
#![recursion_limit = "256"]

#[macro_use]
pub mod macros;

pub mod hook;
pub mod ps2;
pub mod replacements;
pub mod types;

#[cfg(feature = "ui")]
mod ui;

// Dummy exports so that the other side of the equation can safely
// assume these functions are available to be called.
#[cfg(not(feature = "ui"))]
mod ui {
    #[unsafe(export_name = "UpdateUI")]
    extern "C" fn update_ui() {}

    #[unsafe(export_name = "PickThing")]
    extern "C" fn pick_thing(_idx: u16) {}
}

mod w {
    pub use windows::Win32::System::LibraryLoader::*;
    pub use windows::Win32::System::Memory::*;
}

use std::sync::Once;

use eyre::WrapErr;
use windows_strings::s;

#[unsafe(export_name = "InstallHooks")]
pub unsafe extern "C" fn install_hooks() {
    static ONCE: Once = Once::new();

    ONCE.call_once(|| {
        if let Err(e) = install_hooks_impl() {
            println!("oh no! {e:?}");
        }
    });
}

#[unsafe(export_name = "SetDeltaTime")]
pub unsafe extern "C" fn update_values(delta: f32) {
    replacements::values::set_dt(delta);
}

fn install_hooks_impl() -> eyre::Result<()> {
    unsafe {
        let module = w::LoadLibraryA(s!("katamari_Data/Plugins/PS2KatamariSimulation.dll"))
            .wrap_err("LoadLibrary failed")?;

        ps2::link(module);
        hook::entry::init(module);

        let dll = ps2::DLL;

        macro_rules! patches {
            () => {};

            (
                $func:ident => {}
                $($tt:tt)*
            ) => {
                hook::skip(dll.$func)?;
                patches!($($tt)*);
            };

            (
                $func:ident => $($replacement:ident)::*;
                $($tt:tt)*
            ) => {
                hook::install(dll.$func, replacements::$($replacement)::* as _)?;
                patches!($($tt)*);
            };

            (
                $func:ident[$range:expr] => {}
                $($tt:tt)*
            ) => {
                hook::skip_range(dll.$func, $range)?;
                patches!($($tt)*);
            };

            (
                $func:ident[$range:expr] => $($replacement:ident)::*;
                $($tt:tt)*
            ) => {
                hook::patch(dll.$func, $range, replacements::$($replacement)::* as _)?;
                patches!($($tt)*);
            };

            // EXTREMELY sketchy, but handy for e.g. changing a Jcc into another Jcc
            (
                $func:ident[$offset:literal $(+ $extra:literal)? ..] => $replacement:expr;
                $($tt:tt)*
            ) => {
                hook::raw_patch(dll.$func, $offset $(+ $extra)?, bytemuck::bytes_of(&$replacement))?;
                patches!($($tt)*);
            };
        }

        patches! {
            katamari_physics_big_kahuna[0x41b..0x4e8] => movement::rolling_position;
            katamari_physics_big_kahuna[0x3ac..0x40e] => movement::climbing_position;
            katamari_physics_climb[0x259..0x2d7] => movement::climbing_ascent;

            katamari_physics_roll[0x254..0x25b] => movement::spin_amount;
            katamari_pivot[0x17a..0x182] => movement::bumpy_ride;
            katamari_physics_big_kahuna[0x146..0x15f] => movement::friction;
            speed_thing_2[0xe12..0xe2d] => movement::push_force;

            katamari_bump_thing[0x375..0x37a] => movement::bump_velocity;

            do_katamari_physics[0x352..0x362] => abilities::spindash_gain_power;
            do_katamari_physics[0x3d5..0x3da] => abilities::spindash_spinning;

            katamari_physics_climb[0xb9..0xd2] => movement::climb_ascent_timer;
            katamari_physics_climb[0xd2..] => [0x0F, 0x8E]; // JNE -> JNG
            katamari_physics_climb[0x15..0x26] => movement::climb_sustain_timer;
            katamari_physics_climb[0x7F..0x90] => movement::climb_sustain_timer;

            copy_matrix => copy_matrix;

            calculate_gravity[0x688..0x820] => movement::uphill;
            calculate_gravity[0x607..0x683] => movement::downhill;

            // Steering with both sticks.
            handle_turn[0xb5..0xba] => movement::steer_rcx;

            // Steering with one stick.
            actually_apply_player_input_force_2[0x37c..0x381] => movement::steer_rbx;
            actually_apply_player_input_force_2[0x2d4..0x2d9] => movement::steer_rbx;
            actually_apply_player_input_force_2[0x222..0x227] => movement::steer_rbx;
            actually_apply_player_input_force_2[0x16c..0x171] => movement::steer_rbx;

            // Steering while still pushing forward.
            gentle_steering[0x25d..0x262] => movement::steer_rbx;

            ontick_update_angle_guy[0x109..0x14e] => cacophony::sfx_npc_approaching;

            //prince_flip[0x305..0x35b] => prince_flip;
            tick_player[0x13a..0x141] => dash::stamina_gain;
            prince_handle_dash[0x16f..0x176] => dash::stamina_drain;
            prince_handle_dash[0x3f9..0x404] => dash::update_dash_input_timer;
            speed_thing_2[0x143..0x21f] => dash::dash_state_machine;
            prince_handle_dash[0x14c..0x153] => dash::multiplayer_dash_input_window;

            speed_thing_2[0x42a..0x42f] => movement::turn_radius;

            prince_exhausted[0x51..0x58] => dash::prince_exhausted;

            splash[0x3c1..0x5ad] => cacophony::splash;

            // This one only fixes the global cooldown; the NPC also has a longer local cooldown.
            gunshot[0x1cb..0x1eb] => cacophony::bang;
            // This handles the local cooldown
            gunshot[0xd5..0x10a] => cacophony::bang2;
            // Remove a weird remaining increment of the counter, since my code handles it
            gunshot[0x1bb..0x1be] => {}

            //do_katamari_physics[0x24c..0x25e] => gravitee;
            //do_katamari_physics[0x40e..0x413] => {}
            //do_katamari_physics[0x4f1..0x4fd] => {}
            //some_sort_of_collision_guy[0xd..0x7e] => good_night;

            calculate_gravity[0x12d..0x135] => movement::airborne_gravity;

            npc_40bb0[0x7a..0x8a] => things::melon_spin;

            thing_animal_motion[0x9e..0xe5] => things::animal_walk;

            camera_bear_cow_orbit[0xd7..0xe7] => camera::orbit_a;
            camera_versus_winner_orbit[0x55..0x67] => camera::orbit_b;
            camera_animate[0x2b4..0x2c4] => camera::zoom_out;

            tick_player[0x113..0x121] => abilities::flip_duration;
            prince_flip[0x62..0x73] => abilities::flip_timer;
            prince_flip[0x303..] => [0x7F]; // JNZ -> JG

            camera_update_katamari_view[0x489..0x48f] => abilities::katamari_view_ascend;
            camera_update_katamari_view[0xed..0xf5] => abilities::katamari_view_descend;
            camera_set_view_mode[0x286..] => 666_i32; // mov eax, 20 -> mov eax, 666 (frames -> ms)

            camera_update_xc500[0x8e..0x96] => camera::size_threshold_animation_timer;
            camera_update_xc500[0x1b5..0x1bd] => camera::size_threshold_animation_spin;
            // This one adds the vector @ 0x50 to the vector @ 0x10
            camera_update_xc500[0xa5..0xf5] => camera::size_threshold_animation_other_zoom;
            // This one adds the vector @ 0x40 to the vector @ 0x00
            camera_update_xc500[0xf5..0x148] => camera::size_threshold_animation_zoom;

            // Cheat code to pick up things of any size
            //katamari_queue_things_for_pickup[0x2d7..0x2d9] => {}
            // Cheat code to always flip things and never collect directly
            //katamari_x28640[0x19b..0x1d6] => {}

            // TODO: I had a very hard time figuring out where to put this hook,
            // but that was before the 12->5 byte hook shrink.
            // I can probably find a better spot now.
            katamari_sink_things[0x2f..0x34] => things::sink_rate;

            thing_hop[0x10..0x29] => things::hop_timer_update;
            thing_hop_turn[0x69..0x76] => things::hop_angle_update;
            thing_hop_apply_velocity[0xd9..0x14a] => things::hop_position_update;
            thing_hop_apply_velocity[0x8c..0x94] => things::hop_gravity;

            // There's probably still more to be done in this function, given its sheer scale,
            // but this is pretty good for now.
            thing_freefall[0x12..0x1c] => things::freefall_gravity;
            thing_freefall[0x2b..0x5e] => things::freefall_pos;
            thing_freefall[0xb7..0xc0] => things::freefall_spin_a;
            thing_freefall[0x267..0x270] => things::freefall_spin_b;
            thing_freefall[0x2f1..0x305] => things::freefall_spin_c;
            // for some reason the compiler was extra silly
            // and put the x component addition after both ends of the branch
            // my code just does it alongside the others
            //
            // it also... only writes the position after both ends of the branch
            // no thanks, I'll do it unconditionally rather than biconditionally
            thing_freefall[0x64..0x9f] => {}   // the first branch
            thing_freefall[0x201..0x211] => {} // the second branch, before some unrelated stuff
            thing_freefall[0x21b..0x246] => {} // the second branch, after the unrelated stuff

            thing_random_hop_main[0x60..0x88] => things::random_hop_motion;
            thing_reset_hop_timer[0x2c..0x33] => things::hop_timer_reset;

            // Hooks below this line still need to be analyzed for improvements with the smaller hook.

            thing_animal_state_3_turn[0x61..0x6e] => things::animal_angle_move_towards;
            // This isn't working. There's something missing. More blood must be shed.
            thing_train_x391b0[0x48..0x56] => things::train_angle_move_towards;

            thing_flee_state_1[0x80+1..] => 333_u32; // start flee timer 1
            thing_flee_state_2[0x55..0x5f] => things::update_flee_timer_1;
            thing_flee_state_2[0x64..0x70] => things::start_flee_timer_2;
            thing_flee_state_4[0x3f..0x47] => things::update_flee_timer_2;

            f32_angle_move_towards[0..0x11] => things::angle_move_towards;
            pursuit_angle_move_towards[0..0x12] => things::pursuit_angle_move_towards;

            // Hooks below this line are newer than the "hooks above this line" comment.

            thing_machine_22_state_2[0x30..0x35] => things::sine_bob;
            thing_machine_22_state_3[0x58..0x7f] => things::other_hop_gravity;

            thing_machine_22_state_3[0x386..0x38c] => things::other_hop_timer_check;
            thing_machine_22_state_3[0x38e..0x393] => things::other_hop_timer_update;
            thing_machine_22_state_3[0x449..0x451] => things::other_hop_timer_start;

            thing_machine_22_state_6[0x1b1..0x1bb] => things::elevator_timer_reset;
            thing_machine_22_state_6[0x164..0x16a] => things::elevator_timer_reset;
            thing_machine_22_state_6[0xbf..0xc4] => things::elevator_timer_update;
            thing_machine_22_state_6[0xd4..0xd9] => things::elevator_height_update;
            thing_machine_22_state_6[0x63..0x68] => things::elevator_height_update;

            thing_machine_20_state_1[0x51..0x59] => things::teddy_bear_bowl_spin;

            thing_basic_gravity[0x21..0xac] => things::basic_gravity;

            thing_wobble_state_1[0x5cb..0x5d0] => things::wobble_rate;

            camera_main[0x1a3..0x1bb] => camera::angel_zoom;
            camera_main[0x22a..0x231] => camera::angel_fade;

            katamari_attach_thing_x28ef0[0x562..0x56a] => stereo_haptics::pickup_hook;
            katamari_collide_with_wall[0x7e2..0x7e9] => stereo_haptics::wall_bump_hook;
            katamari_bump_thing[0..5] => stereo_haptics::thing_bump_hook;
            katamari_flip_thing[0x2f..0x34] => stereo_haptics::thing_flip_hook;

            thing_start_rng_timer[0x59..0x63] => things::start_rng_timer;
            thing_random_hop_main[0x1bf..0x1c7] => things::fish_timer_decrement;

            thing_x36a10[0x2c5..0x2cb] => things::getup_timer_update;
            thing_x36a10[0x32f..0x336] => things::getup_timer_check;

            thing_getup_flee_state_0[0x2d..0x35] => things::flee_timer_start;
            thing_getup_flee_state_1[0x3f..0x46] => things::flee_timer_update;
            thing_getup_flee_state_2[0x385..] => 667_u32; // 20 ticks -> ⅔ seconds

            thing_getup_flee_state_3a[0x9..0x10] => things::flee_timer_update;
            thing_getup_flee_state_3b[0x9..0x10] => things::flee_timer_update;
            thing_getup_flee_state_3a[0x14+1..] => 1000_u32; // 1 second
            thing_getup_flee_state_3b[0x14+1..] => 1000_u32;

            thing_getup_flee_state_4a[0x4f2..0x4f7] => things::flee_timer_update_state4;
            thing_getup_flee_state_4b[0x1ab..0x1b0] => things::flee_timer_update_state4;

            katamari_queue_things_for_pickup[0x20f..0x215] => things::update_collision_cooldown;

            katamari_flip_thing[0x174+1..] => 1000_u32; // NPC collision cooldown: 1 second
            x27170[0x233+1..] => 333_u32;               // NPC collision cooldown: ⅓ second
            katamari_bump_thing[0x181+1..] => 333_u32;
            katamari_hit_test[0x15d0+1..] => 333_u32;
            katamari_x28e00[0xd5+1..] => 167_u32;       // NPC collision cooldown: ⅙ second

            thing_getup_flee_state_1[0..0x17] => things::spin_before_getup;
            thing_getup_flee_state_2[0x3a..0x64] => things::getup_hop_gravity;
            thing_getup_flee_state_2[0xe1..0x100] => things::getup_flip;

            thing_getup_flee_state_4a[0x9e..0x10c] => things::flee_velocity_1;
            thing_getup_flee_state_4a[0x137..0x19d] => things::flee_velocity_2;
            thing_getup_flee_state_4b[0x6f..0x74] => things::flee_velocity_3;
            thing_getup_flee_state_4b[0x49..0x4e] => things::flee_velocity_spin;

            thing_spin_x49d40[0x23..0x28] => things::jumboman_spin;
            thing_windmill_spin_x39900[0x1c..0x21] => things::windmill_spin;

            // The behavior code for NPCs that walk laps around a fixed path
            // is already delta-timed in the vanilla game, but in a strange way
            // that leads to a bug where they turn too slowly
            // (which may look like they're "snapping" to each angle if making several small turns in a row)
            //
            // This happens because it uses the "current" delta-time to decide the movement speed
            // and turn rate, but then also delta-times the rotation update that *uses* the turn rate.
            // Currently, the movement speed remains using this original approach,
            // while these patches remove the time factor from the "turn rate" value.
            // Ideally the movement would use a consistent design with the rest of my changes,
            // but as long as it's not obviously janky, it's fine, at least for now.
            thing_walk_cycle_x39440[0x25b..0x260] => things::remove_redundant_delta;
            thing_walk_cycle_x42080[0x244..0x249] => things::remove_redundant_delta;

            thing_pendulum[0x23..0x28] => things::pendulum;
            thing_wrecking_ball_pendulum[0x47..0x4c] => things::wrecking_ball;

            thing_bird_state_0_init[0x62..0x70] => things::bird_start_rng_timer_rax;
            thing_bird_state_1_idle[0x2b..0x32] => things::bird_update_timer_rbx;
            thing_bird_state_4_ascend[0x117+1..] => 5000_u32; // 150 ticks -> 5 seconds
            thing_bird_state_5_stay_in_sky[0x1e..0x25] => things::bird_update_timer_rdx;
            thing_bird_state_5_stay_in_sky[0x2b+1..] => 5000_u32;
            thing_bird_state_5_stay_in_sky[0x56..0x67] => things::bird_start_rng_timer_rdx;
            thing_bird_state_6_begin_descent[0x25..0x2c] => things::bird_update_timer_rbx;

            thing_bird_state_4_ascend[0x36..0x3b] => things::bird_ascend_vertical_a;
            thing_bird_state_4_ascend[0x4c..0x51] => things::bird_ascend_horizontal;
            thing_bird_state_4_ascend[0x8e..0x93] => things::bird_ascend_vertical_b;

            thing_bird_state_7_descend[0x34..0x95] => things::bird_descend_horizontal;
            thing_bird_state_7_descend[0xf8..0x100] => things::bird_descend_vertical_a;
            thing_bird_state_7_descend[0xc1..0xc6] => things::bird_descend_vertical_b;

            // When the "full walk timer" hits zero, the NPC will make a turn.
            thing_animal_state_0_start[0x3e3..0x3fb] => things::animal_start_walk_timer;
            thing_animal_state_0_start[0x455+1..] => 1000_u32;
            thing_animal_x3bfb0[0x91..0x98] => things::animal_update_full_walk_timer;
            thing_animal_x3bfb0[0x39..0x64] => things::animal_restart_walk_timer;

            // When the "partial walk timer" hits zero, the NPC will stop walking for a brief period.
            // (This period is controlled by the same variable.)
            thing_animal_state_1_walk[0x89..0x90] => things::animal_update_partial_walk_timer;
            thing_animal_state_1_walk[0x32..0x39] => things::animal_update_partial_walk_timer;
            thing_animal_state_1_walk[0xae..0xcf] => things::animal_reset_partial_walk_timer_30;
            thing_animal_state_1_walk[0x5a..0x7b] => things::animal_reset_partial_walk_timer_60;

            thing_scarecrow_sway[0x1ed..0x1f2] => things::scarecrow_sway;

            prince_read_sticks[0x101..0x107] => movement::prince_bump_timer_update;
            camera_animate[0x291..0x298] => camera::camera_bump_timer_update;
            prince_forced_turn[0x119..0x121] => movement::prince_forced_turn;

            thing_animal_x31f90[0x34..0x39] => things::animal_flee_turn;

            camera_credits_update[0x1de..0x1e6] => camera::credits_zoom;
            camera_credits_update[0xe9..0xf1] => camera::credits_zoom;

            init[0x251..0x258] => camera::credits_timer_start_a;
            camera_credits_update[0x1ca..0x1d1] => camera::credits_timer_start_b;
            camera_credits_update[0xcc..0xd3] => camera::credits_timer_zero;
            camera_credits_update[0x199..0x1a0] => camera::credits_timer_zero;

            camera_credits_update[0xa6..0xba] => camera::credits_timer_update;
            camera_credits_update[0x189..0x191] => camera::credits_timer_update;

            katamari_physics_big_kahuna[0x37c..0x383] => movement::credits_sphere_walk;

            // TODO: does this go on other call sites? there are four in total
            thing_freefall[0x469..0x46e] => things::thing_bounce;
            thing_collide_with_other_thing[0x215..0x21a] => things::thing_bounce;

            katamari_collide_with_wall[0xda..0xe2] => movement::fall_time_check;
            katamari_physics_x14c80[0x593..0x59d] => movement::air_time_increment;
            katamari_physics_x14c80[0x60f..0x619] => movement::air_time_increment;
            katamari_physics_x14c80[0x5b2..0x5b9] => movement::fall_time_increment;
            katamari_physics_x14c80[0x62e..0x635] => movement::fall_time_increment;

            thing_kickball_state_1[0x3b..0xd1] => things::kickball_position;
            thing_kickball_state_1[0x15e..0x166] => things::kickball_rotation;
            thing_kickball_physics[0x10b..0x14b] => things::kickball_deceleration;
        }

        replacements::values::PTR_THING_GRAVITY = ps2::raw::thing_gravity();
        replacements::values::PTR_CREDITS_TIMER = ps2::raw::credits_timer();

        macro_rules! ticks_to_millis {
            ($ptr:expr) => {
                let v = $ptr;
                *v = (*v * 1000) / 30;
            };
        }

        ticks_to_millis!(ps2::raw::climb_sustain_limit());
        ticks_to_millis!(ps2::raw::air_time_min());
        ticks_to_millis!(ps2::raw::fall_time_min());
        ticks_to_millis!(ps2::raw::fall_time_max());
        ticks_to_millis!(ps2::bump_timers(0));
        ticks_to_millis!(ps2::bump_timers(1));

        hook::postfix(
            dll.initialize_princes,
            0x3e3,
            replacements::prince_post_init,
        )?;
    }

    Ok(())
}
