#![feature(slice_from_ptr_range)]
#![feature(portable_simd)]

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
                $func:ident[$offset:literal..] => $replacement:expr;
                $($tt:tt)*
            ) => {
                hook::raw_patch(dll.$func, $offset, &$replacement)?;
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

            // This causes the giant watermelons to stop rotating their yaw for some reason.
            //copy_matrix => copy_matrix;

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

            speed_thing_2[0x42a..0x42f] => movement::turn_radius;

            prince_exhausted[0x51..0x58] => dash::prince_exhausted;

            splash[0x3c1..0x5ad] => cacophony::splash;

            // This one only fixes the global cooldown; the NPC also has a longer local cooldown.
            gunshot[0x1cb..0x2c7] => cacophony::bang;
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
            camera_set_view_mode[0x286..] => 666_i32.to_ne_bytes(); // mov eax, 20 -> mov eax, 666 (frames -> ms)

            camera_update_xc500[0x8e..0x96] => camera::size_threshold_animation_timer;
            camera_update_xc500[0x1b5..0x1bd] => camera::size_threshold_animation_spin;
            // This one adds the vector @ 0x50 to the vector @ 0x10
            camera_update_xc500[0xa5..0xf5] => camera::size_threshold_animation_other_zoom;
            // This one adds the vector @ 0x40 to the vector @ 0x00
            camera_update_xc500[0xf5..0x148] => camera::size_threshold_animation_zoom;

            // Cheat code to pick up things of any size
            // katamari_queue_things_for_pickup[0x2d7..0x2d9] => {}

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
            thing_freefall[0x46..0x5e] => things::freefall_pos;
            thing_freefall[0xb7..0xc0] => things::freefall_spin_a;
            thing_freefall[0x267..0x270] => things::freefall_spin_b;
            thing_freefall[0x2f1..0x305] => things::freefall_spin_c;
            // for some reason the compiler was extra silly
            // and put the x component addition after both ends of the branch
            // my code just does it alongside the others
            //
            // this is another case where I should probably get this using VFMADDPS
            thing_freefall[0x64..0x74] => {}
            thing_freefall[0x201..0x211] => {}

            thing_random_hop_main[0x60..0x88] => things::random_hop_motion;
            thing_reset_hop_timer[0x2c..0x33] => things::hop_timer_reset;

            // Hooks below this line still need to be analyzed for improvements with the smaller hook.

            thing_animal_state_3_turn[0x61..0x6e] => things::animal_angle_move_towards;
            // This isn't working. There's something missing. More blood must be shed.
            thing_train_x391b0[0x48..0x56] => things::train_angle_move_towards;

            thing_flee_state_2[0x64..0x70] => things::start_flee_timer;
            thing_flee_state_4[0x3f..0x47] => things::update_flee_timer;

            f32_angle_move_towards[0..0x11] => things::angle_move_towards;
            pursuit_angle_move_towards[0..0x12] => things::pursuit_angle_move_towards;

            // Hooks below this line are newer than the "hooks above this line" comment.

            thing_machine_22_state_2[0x30..0x35] => things::sine_bob;
            thing_machine_22_state_3[0x58..0x7f] => things::other_hop_gravity;

            thing_machine_22_state_6[0x1b1..0x1bb] => things::elevator_timer_reset;
            thing_machine_22_state_6[0x164..0x16a] => things::elevator_timer_reset;
            thing_machine_22_state_6[0xbf..0xc4] => things::elevator_timer_update;
            thing_machine_22_state_6[0xd4..0xd9] => things::elevator_height_update;
            thing_machine_22_state_6[0x63..0x68] => things::elevator_height_update;

            thing_machine_20_state_1[0x51..0x59] => things::teddy_bear_bowl_spin;

            katamari_attach_thing_x28ef0[0x562..0x56a] => stereo_haptics::pickup_hook;
            katamari_collide_with_wall[0x7e2..0x7e9] => stereo_haptics::wall_bump_hook;
            katamari_bump_thing[0..5] => stereo_haptics::thing_bump_hook;
        }

        replacements::values::PTR_THING_GRAVITY = ps2::raw::thing_gravity();

        {
            let n = &mut *ps2::raw::climb_sustain_limit();
            *n = (*n * 1000) / 30;
        }

        hook::postfix(
            dll.initialize_princes,
            0x3e3,
            replacements::prince_post_init,
        )?;
    }

    Ok(())
}
