#![feature(const_slice_from_ptr_range)]
#![feature(slice_from_ptr_range)]
#![feature(portable_simd)]

#[macro_use]
pub mod macros;

pub mod hook;
pub mod ps2;
pub mod replacements;
pub mod types;

mod w {
    pub use windows::Win32::Foundation::*;
    pub use windows::Win32::System::LibraryLoader::*;
    pub use windows::Win32::System::Memory::*;
    pub use windows::Win32::System::SystemServices::*;
}

use eyre::WrapErr;
use windows_strings::s;

#[unsafe(export_name = "DllMain")]
pub extern "system" fn dll_main(
    dll_module: w::HINSTANCE,
    call_reason: u32,
    reserved: *mut (),
) -> bool {
    match call_reason {
        w::DLL_PROCESS_ATTACH => {
            println!("{:?}", dll_module);
            if let Err(e) = on_attach(dll_module) {
                println!("oh no!");
                println!("{e:?}");
            }
        }
        w::DLL_PROCESS_DETACH => println!("detach"),
        w::DLL_THREAD_ATTACH => (),
        w::DLL_THREAD_DETACH => (),
        _ => println!("hello from DLL! {dll_module:?} {call_reason:?} {reserved:?}"),
    }
    true
}

fn on_attach(dll_module: w::HINSTANCE) -> eyre::Result<()> {
    unsafe {
        w::DisableThreadLibraryCalls(dll_module.into())?;

        let module = w::LoadLibraryA(s!("katamari_Data/Plugins/PS2KatamariSimulation.dll"))
            .wrap_err("LoadLibrary failed")?;

        ps2::link(module);

        let dll = ps2::DLL.functions;

        hook::patch(
            dll.katamari_physics_big_kahuna,
            0x41B..0x445,
            replacements::normal_motion_branch_first_part,
        )?;

        hook::install(dll.copy_matrix, replacements::copy_matrix as _)?;

        hook::patch(
            dll.katamari_physics_big_kahuna,
            0x597..0x5a7,
            replacements::standing_on_prop_branch,
        )?;

        hook::patch(
            dll.katamari_physics_prop_collision,
            0xA0..0xD1,
            replacements::prop_terrain_collision,
        )?;

        // A block I tried to disable to discover gravity
        // hook::patch(dll.calculate_gravity_and_some_other_forces, 0x79a..0x7ae, replacements::gravity)?;
        // Two more, of twin nature
        // hook::patch(dll.calculate_gravity_and_some_other_forces, 0x7fe..0x819, replacements::gravity)?;
        // hook::patch(dll.calculate_gravity_and_some_other_forces, 0x661..0x673, replacements::gravity)?;

        // The one that actually worked, but not the code I'm actually going to patch
        // hook::patch(dll.calculate_gravity_and_some_other_forces, 0x12d..0x186, replacements::gravity)?;

        hook::patch(
            dll.calculate_gravity_and_some_other_forces,
            0xd4..0xfd,
            replacements::gravity,
        )?;

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
    }

    Ok(())
}
