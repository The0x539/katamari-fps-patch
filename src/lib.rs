#![feature(const_slice_from_ptr_range)]
#![feature(slice_from_ptr_range)]

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

        /*
        let detour = RawDetour::new(
            ps2::DLL.katamari_physics_big_kahuna as *const (),
            replacements::null_big_kahuna as *const (),
        )
        .wrap_err("Failed to construct detour")?;

        detour.enable().wrap_err("Failed to enable detour")?;
        println!("Detour supposedly installed");
        */

        hook::patch(
            ps2::DLL.katamari_physics_big_kahuna as _,
            0x41B..0x445,
            replacements::normal_motion_branch_first_part,
        )?;
    }

    Ok(())
}
