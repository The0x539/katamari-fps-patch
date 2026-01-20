#![feature(const_slice_from_ptr_range)]
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
                hook::patch(dll.$func, $range, replacements::$($replacement)::*)?;
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

            katamari_physics_roll[0x254..0x260] => movement::spin_amount;
            // TODO: Emit a more visible complaint when the patched range is smaller than needed for the hook.
            katamari_pivot[0x17a..0x187] => movement::bumpy_ride;
            katamari_physics_big_kahuna[0x146..0x15f] => movement::friction;
            speed_thing_2[0xe12..0xe2d] => movement::push_force;

            katamari_bump_thing[0x375..0x382] => movement::bump_velocity;

            do_katamari_physics[0x3c9..0x3da] => abilities::spindash_spinning;

            katamari_physics_climb[0xb9..0xd2] => movement::climb_ascent_timer;
            katamari_physics_climb[0xd2..] => [0x0F, 0x8E]; // JNE -> JNG
            katamari_physics_climb[0x15..0x26] => movement::climb_sustain_timer;
            katamari_physics_climb[0x7F..0x90] => movement::climb_sustain_timer;

            // This causes the giant watermelons to stop rotating their yaw for some reason.
            //copy_matrix => copy_matrix;

            calculate_gravity[0x688..0x820] => uphill;
            calculate_gravity[0x607..0x683] => downhill;

            // TODO: determine if it would be better to just edit g_katamariRot.
            // Does anyone set it? Or does the game use it as a constant?
            handle_turn[0xa8..0xba] => fast_steer;

            // The instructions in play here are identical in all four of these cases,
            // aside from a RIP-relative offset for loading g_katamariRot.
            actually_apply_player_input_force_2[0x36f..0x381] => slow_steer;
            actually_apply_player_input_force_2[0x2c7..0x2d9] => slow_steer;
            actually_apply_player_input_force_2[0x215..0x227] => slow_steer;
            actually_apply_player_input_force_2[0x15f..0x171] => slow_steer;

            gentle_steering [0x248..0x255] => gentle_steer;

            ontick_update_angle_guy[0x109..0x14e] => sfx_npc_approaching;

            //prince_flip[0x305..0x35b] => prince_flip;
            tick_player[0x131..0x188] => stamina_gain;
            prince_handle_dash[0x16f..0x17d] => stamina_drain;
            prince_handle_dash[0x404..0x412] => update_dash_input_timer;
            speed_thing_2[0x143..0x21f] => dash_state_machine;

            speed_thing_2[0x42a..0x438] => turn_radius;

            // For some reason, replacing a small part of this function (0x48..0x6a)
            // just broke it entirely.
            prince_exhausted => prince_exhausted;

            splash[0x3c1..0x5ad] => splash;

            // This one only fixes the global cooldown; the NPC also has a longer local cooldown.
            gunshot[0x1cb..0x2c7] => bang;
            // This handles the local cooldown
            gunshot[0xd5..0x10a] => bang2;
            // Remove a weird remaining increment of the counter, since my code handles it
            gunshot[0x1bb..0x1be] => {}

            //do_katamari_physics[0x24c..0x25e] => gravitee;
            //do_katamari_physics[0x40e..0x413] => {}
            //do_katamari_physics[0x4f1..0x4fd] => {}
            //some_sort_of_collision_guy[0xd..0x7e] => good_night;

            calculate_gravity[0x12d..0x13d] => airborne_gravity;

            npc_40bb0[0x7a..0x8a] => melon_spin;

            npc_update_animal_position[0x9e..0xe5] => animal_walk;

            camera_bear_cow_orbit[0xd7..0xe7] => camera::orbit_a;
            camera_versus_winner_orbit[0x55..0x67] => camera::orbit_b;
            camera_animate[0x2b4..0x2c4] => camera::zoom_out;

            tick_player[0x113..0x121] => abilities::flip_duration;
            prince_flip[0x62..0x73] => abilities::flip_timer;
            prince_flip[0x303..] => [0x7F]; // JNZ -> JG
            // TODO: go back and figure out if any other branch fixes would be simplified by the "raw" patch

            camera_update_katamari_view[0x489..0x495] => abilities::katamari_view_ascend;
            camera_update_katamari_view[0xe7..0xf5] => abilities::katamari_view_descend;
            camera_set_view_mode[0x286..] => 666_i32.to_ne_bytes(); // mov eax, 20 -> mov eax, 666 (frames -> ms)

            // Cheat code to pick up things of any size
            // katamari_queue_things_for_pickup[0x2d7..0x2d9] => {}

            katamari_sink_things[0x2f..0x3b] => things::sink_rate;

            thing_hop[0x10..0x29] => things::hop_timer_update;
            thing_hop_turn[0x69..0x76] => things::hop_angle_update;
            thing_hop_apply_velocity[0xd9..0x14a] => things::hop_position_update;
            thing_hop_apply_velocity[0x8c..0x9c] => things::hop_gravity;

            // There's probably still more to be done in this function, given its sheer scale,
            // but this is pretty good for now.
            thing_freefall[0xa..0x1c] => things::freefall_gravity;
            thing_freefall[0x46..0x5e] => things::freefall_pos;
            thing_freefall[0xa5..0xc0] => things::freefall_spin_a;
            thing_freefall[0x255..0x270] => things::freefall_spin_b;
            thing_freefall[0x2f1..0x305] => things::freefall_spin_c;
            // for some reason the compiler was extra silly
            // and put the x component addition after both ends of the branch
            // my code just does it alongside the others
            //
            // this is another case where I should probably get this using VFMADDPS
            thing_freefall[0x64..0x74] => {}
            thing_freefall[0x201..0x211] => {}

            thing_random_hop_main[0x60..0x88] => things::random_hop_motion;
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

        hook::patch_preserving_rax(
            dll.thing_reset_hop_timer,
            0x2c..0x3c,
            replacements::things::hop_timer_reset,
        )?;

        hook::patch_preserving_rax(
            dll.do_katamari_physics,
            0x352..0x362,
            replacements::abilities::spindash_gain_power,
        )?;
    }

    Ok(())
}
