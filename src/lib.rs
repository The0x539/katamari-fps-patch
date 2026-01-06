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

        hook::patch(
            dll.katamari_physics_big_kahuna,
            0x41B..0x445,
            replacements::normal_motion_branch_first_part,
        )?;

        hook::install(dll.copy_matrix, replacements::copy_matrix as _)?;

        // hook::patch(
        //     dll.katamari_physics_big_kahuna,
        //     0x597..0x5a7,
        //     replacements::standing_on_prop_branch,
        // )?;

        // hook::patch(
        //     dll.katamari_physics_prop_collision,
        //     0xA0..0xD1,
        //     replacements::prop_terrain_collision,
        // )?;

        // TODO: Emit a more visible complaint when the patched range is smaller than needed for the hook.
        hook::patch(dll.katamari_pivot, 0x17a..0x187, replacements::bumpy_ride)?;

        // A block I tried to disable to discover gravity
        // hook::patch(dll.calculate_gravity_and_some_other_forces, 0x79a..0x7ae, replacements::gravity)?;
        // Two more, of twin nature
        // hook::patch(dll.calculate_gravity_and_some_other_forces, 0x7fe..0x819, replacements::gravity)?;
        // hook::patch(dll.calculate_gravity_and_some_other_forces, 0x661..0x673, replacements::gravity)?;

        // The one that actually worked, but not the code I'm actually going to patch
        // hook::patch(dll.calculate_gravity_and_some_other_forces, 0x12d..0x186, replacements::gravity)?;

        // hook::patch(
        //     dll.calculate_gravity_and_some_other_forces,
        //     0xd4..0xfd,
        //     replacements::gravity,
        // )?;

        // TODO: determine if it would be better to just edit g_katamariRot.
        // Does anyone set it? Or does the game use it as a constant?
        hook::patch(dll.handle_turn, 0xa8..0xba, replacements::fast_steer)?;

        // The instructions in play here are identical in all four of these cases,
        // aside from a RIP-relative offset for loading g_katamariRot.
        for span in [0x36f..0x381, 0x2c7..0x2d9, 0x215..0x227, 0x15f..0x171] {
            hook::patch(
                dll.actually_apply_player_input_force_2,
                span,
                replacements::slow_steer,
            )?;
        }

        hook::patch(
            dll.gentle_steering,
            0x248..0x255,
            replacements::gentle_steer,
        )?;

        hook::patch(
            dll.ontick_update_angle_guy,
            0x109..0x14e,
            replacements::sfx_npc_approaching,
        )?;

        // hook::patch(dll.prince_flip, 0x305..0x35b, replacements::prince_flip)?;

        hook::patch(dll.tick_player, 0x131..0x188, replacements::stamina_gain)?;
        hook::patch(
            dll.prince_handle_dash,
            0x16f..0x17d,
            replacements::stamina_drain,
        )?;

        hook::patch(
            dll.prince_handle_dash,
            0x404..0x412,
            replacements::update_dash_input_timer,
        )?;

        hook::patch(
            dll.speed_thing_2,
            0x143..0x21f,
            replacements::dash_state_machine,
        )?;

        hook::postfix(
            dll.initialize_princes,
            0x3e3,
            replacements::prince_post_init,
        )?;

        // For some reason, replacing a small part of this function (0x48..0x6a)
        // just broke it entirely.
        hook::install(dll.prince_exhausted, replacements::prince_exhausted as _)?;

        hook::patch(dll.splash, 0x3c1..0x5ad, replacements::splash)?;

        // This one only fixes the global cooldown; the NPC also has a longer local cooldown.
        hook::patch(dll.gunshot, 0x1cb..0x2c7, replacements::bang)?;

        // hook::patch(
        //     dll.do_katamari_physics,
        //     0x24c..0x25e,
        //     replacements::gravitee,
        // )?;

        // hook::skip_range(dll.do_katamari_physics, 0x40e..0x413)?;
        // hook::skip_range(dll.do_katamari_physics, 0x4f1..0x4fd)?;

        // hook::patch(
        //     dll.some_sort_of_collision_guy,
        //     0xd..0x7e,
        //     replacements::good_night,
        // )?;

        hook::patch(
            dll.calculate_gravity_and_some_other_forces,
            0x12d..0x13d,
            replacements::gravity_mark3,
        )?;
    }

    Ok(())
}
