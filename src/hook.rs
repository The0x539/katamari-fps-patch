use eyre::WrapErr;

use crate::w;

pub unsafe fn install(target: *mut (), detour: *const ()) -> eyre::Result<()> {
    unsafe {
        let hook_code = hook_asm_bytes();
        let target_code = std::ptr::slice_from_raw_parts_mut(target.cast(), hook_code.len());

        mprotect(target_code, w::PAGE_READWRITE, || {
            let their_code = &mut *target_code;
            their_code.copy_from_slice(hook_code);
            for i in 0..their_code.len() {
                let window = &mut their_code[i..][..8];
                if window == [0xEE; 8] {
                    window.copy_from_slice(&detour.addr().to_ne_bytes());
                    break;
                }
            }
        })
        .wrap_err("VirtualProtect went wrong")?;
    }

    Ok(())
}

core::arch::global_asm! {
    "hook_start:",
    // needs to be the absolute address of the function I'm jumping to,
    // because we're jumping to it from the other DLL,
    // and both DLLs are subject to separate ASLR.
    // thus, this address has to be computed by regular runtime code.
    "mov rax, 0xEEEEEEEEEEEEEEEE",
    "jmp rax",
    "hook_end:",
}

unsafe extern "C" {
    static hook_start: u8;
    static hook_end: u8;
}

fn hook_asm_bytes() -> &'static [u8] {
    unsafe {
        let start = &raw const hook_start;
        let end = start.with_addr((&raw const hook_end).addr());
        std::slice::from_ptr_range(start..end)
    }
}

unsafe fn mprotect<M, R>(
    memory: *const [M],
    protection: w::PAGE_PROTECTION_FLAGS,
    f: impl FnOnce() -> R,
) -> eyre::Result<R> {
    unsafe {
        let mut old_protection = Default::default();
        let base = memory.cast();
        let len = memory.len() * std::mem::size_of::<M>();

        w::VirtualProtect(base, len, protection, &mut old_protection)?;
        let result = f();
        w::VirtualProtect(base, len, old_protection, &mut old_protection)?;
        Ok(result)
    }
}
