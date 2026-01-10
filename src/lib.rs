#![feature(const_slice_from_ptr_range)]
#![feature(slice_from_ptr_range)]
#![feature(portable_simd)]

#[macro_use]
pub mod macros;

pub mod hook;
pub mod ps2;
pub mod replacements;
pub mod types;
mod ui;

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
                $func:ident => $replacement:ident;
                $($tt:tt)*
            ) => {
                hook::install(dll.$func, replacements::$replacement as _)?;
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
                $func:ident[$range:expr] => $replacement:ident;
                $($tt:tt)*
            ) => {
                hook::patch(dll.$func, $range, replacements::$replacement)?;
                patches!($($tt)*);
            }
        }

        patches! {
            katamari_physics_big_kahuna[0x41B..0x445] => normal_motion_branch_first_part;

            // This causes the giant watermelons to stop rotating their yaw for some reason.
            //copy_matrix => copy_matrix;

            katamari_physics_big_kahuna[0x597..0x5a7] => standing_on_prop_branch;
            //katamari_physics_prop_collision[0xA0..0xD1] => prop_terrain_collision;

            // TODO: Emit a more visible complaint when the patched range is smaller than needed for the hook.
            katamari_pivot[0x17a..0x187] => bumpy_ride;

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

            x20660[0x254..0x260] => spin_amount;
        }

        hook::postfix(
            dll.initialize_princes,
            0x3e3,
            replacements::prince_post_init,
        )?;
    }

    Ok(())
}
