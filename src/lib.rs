#![feature(const_slice_from_ptr_range)]
#![feature(slice_from_ptr_range)]

#[macro_use]
pub mod macros;

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
#[allow(unused_imports)]
use retour::RawDetour;
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

        println!("it's attach time!");
        println!("{:?}", on_attach as *const ());

        let module = w::LoadLibraryA(s!("katamari_Data/Plugins/PS2KatamariSimulation.dll"))
            .wrap_err("LoadLibrary failed")?;

        println!("PS2 module at {module:?}");

        ps2::link(module);
        println!("successfully linked");

        /*
        let detour = RawDetour::new(
            ps2::DLL.katamari_physics_big_kahuna as *const (),
            replacements::null_big_kahuna as *const (),
        )
        .wrap_err("Failed to construct detour")?;

        detour.enable().wrap_err("Failed to enable detour")?;
        println!("Detour supposedly installed");
        */

        let their_kahuna = ps2::DLL.katamari_physics_big_kahuna;
        let my_kahuna = replacements::null_big_kahuna;

        println!("kahuna at {:?}", { ps2::DLL.katamari_physics_big_kahuna });
        println!("replacement at {:?}", my_kahuna as *const ());

        let my_code = insane_asm_bytes();
        println!("my code: {my_code:x?}");
        let their_code =
            std::slice::from_raw_parts_mut(their_kahuna as *mut () as *mut u8, my_code.len());
        println!("their code: {their_code:x?}");
        println!("the asm: {:x?}", insane_asm_bytes());

        let mut old_protect = Default::default();
        w::VirtualProtect(
            their_code.as_ptr().cast(),
            their_code.len(),
            w::PAGE_EXECUTE_READWRITE,
            &mut old_protect,
        )
        .context("first VirtualProtect failed")?;

        println!("old protect: {old_protect:?}");

        their_code.copy_from_slice(my_code);

        for i in 0..their_code.len() {
            let window = &mut their_code[i..][..8];
            if window == [0xEE; 8] {
                window.copy_from_slice(&(my_kahuna as *const ()).addr().to_ne_bytes());
                break;
            }
        }

        println!("their code after: {their_code:x?}");
        println!("that should do it");

        w::VirtualProtect(
            their_code.as_ptr().cast(),
            their_code.len(),
            old_protect,
            &mut old_protect,
        )
        .wrap_err("second VirtualProtect failed")?;
    }

    Ok(())
}

core::arch::global_asm! {
    "insane_start:",
    // needs to be the absolute address of the function I'm jumping to,
    // because we're jumping to it from the other DLL,
    // and both DLLs are subject to separate ASLR.
    // thus, this address has to be computed by regular runtime code.
    "mov rax, 0xEEEEEEEEEEEEEEEE",
    "jmp rax",
    "insane_end:",
}

unsafe extern "C" {
    static insane_start: u8;
    static insane_end: u8;
}

fn insane_asm_bytes() -> &'static [u8] {
    unsafe {
        let start = &raw const insane_start;
        let end = start.with_addr((&raw const insane_end).addr());
        std::slice::from_ptr_range(start..end)
    }
}
