use crate::w;

pub unsafe fn install(target: impl Target, detour: *const u8) -> eyre::Result<()> {
    let target = target.into_target();
    unsafe {
        let hook_code = hook_asm_bytes();
        let target_code = std::ptr::slice_from_raw_parts_mut(target, hook_code.len());

        let i = hook_code
            .windows(8)
            .position(|w| w == [0xEE; 8])
            .unwrap_or(0);

        mprotect(target_code, w::PAGE_READWRITE, || {
            let target_code = &mut *target_code;
            target_code.copy_from_slice(hook_code);
            target_code[i..][..8].copy_from_slice(&detour.addr().to_ne_bytes());
        })?;
    }

    Ok(())
}

pub unsafe fn patch<R>(
    target: impl Target,
    range: std::ops::Range<usize>,
    detour: unsafe extern "C" fn() -> R,
) -> eyre::Result<()> {
    let target = target.into_target();
    unsafe {
        let detour = detour as *const ();
        let patch_code = patch_asm_bytes();
        let target_code = std::ptr::slice_from_raw_parts_mut(target.add(range.start), range.len());

        let i = patch_code
            .windows(8)
            .position(|w| w == [0xEE; 8])
            .unwrap_or(0);

        mprotect(target_code, w::PAGE_READWRITE, || {
            let target_code = &mut *target_code;
            target_code.fill(0x90);
            target_code[..patch_code.len()].copy_from_slice(patch_code);
            target_code[i..][..8].copy_from_slice(&detour.addr().to_ne_bytes());
        })?;
    }

    Ok(())
}

pub unsafe fn raw_patch(
    target: impl Target,
    offset: usize,
    replacement: &[u8],
) -> eyre::Result<()> {
    let target = target.into_target();
    unsafe {
        let dst = std::ptr::slice_from_raw_parts_mut(target.add(offset), replacement.len());
        mprotect(dst, w::PAGE_READWRITE, || {
            (*dst).copy_from_slice(replacement);
        })?;
    }

    Ok(())
}

pub unsafe fn skip(target: impl Target) -> eyre::Result<()> {
    let target = target.into_target();
    unsafe {
        let target_code = std::ptr::slice_from_raw_parts_mut(target, 1);
        mprotect(target_code, w::PAGE_READWRITE, || {
            (*target_code)[0] = 0xc3;
        })?;
    }

    Ok(())
}

pub unsafe fn skip_range(target: impl Target, range: std::ops::Range<usize>) -> eyre::Result<()> {
    let target = target.into_target();

    unsafe {
        let target_code = std::ptr::slice_from_raw_parts_mut(target.add(range.start), range.len());
        mprotect(target_code, w::PAGE_READWRITE, || {
            (*target_code).fill(0x90);
        })?;
    }

    Ok(())
}

/// In its current form, this function is not to be used on a target that uses RAX to return a value.
pub unsafe fn postfix(
    target: impl Target,
    offset: isize,
    detour: unsafe extern "C" fn(),
) -> eyre::Result<()> {
    let target = target.into_target();
    let detour = detour as *const u8;

    unsafe {
        let hook_code = hook_asm_bytes();
        let target_code =
            std::ptr::slice_from_raw_parts_mut(target.offset(offset), hook_code.len());

        let i = hook_code
            .windows(8)
            .position(|w| w == [0xEE; 8])
            .unwrap_or(0);

        mprotect(target_code, w::PAGE_READWRITE, || {
            let target_code = &mut *target_code;
            assert_eq!(target_code[0], 0xc3);
            target_code.copy_from_slice(hook_code);
            target_code[i..][..8].copy_from_slice(&detour.addr().to_ne_bytes());
        })?;
    }

    Ok(())
}

pub unsafe trait Target {
    fn into_target(self) -> *mut u8;
}

unsafe impl<R> Target for extern "win64" fn() -> R {
    fn into_target(self) -> *mut u8 {
        self as _
    }
}

unsafe impl<T, R> Target for extern "win64" fn(T) -> R {
    fn into_target(self) -> *mut u8 {
        self as _
    }
}

unsafe impl<T, U, R> Target for extern "win64" fn(T, U) -> R {
    fn into_target(self) -> *mut u8 {
        self as _
    }
}

unsafe impl<T, U, V, R> Target for extern "win64" fn(T, U, V) -> R {
    fn into_target(self) -> *mut u8 {
        self as _
    }
}

unsafe extern "C" {
    static hook_start: u8;
    static hook_end: u8;
    static patch_start: u8;
    static patch_end: u8;
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

    "patch_start:",
    "mov rax, 0xEEEEEEEEEEEEEEEE",
    "call rax",
    "patch_end:",
}

fn hook_asm_bytes() -> &'static [u8] {
    unsafe {
        let start = &raw const hook_start;
        let end = start.with_addr((&raw const hook_end).addr());
        std::slice::from_ptr_range(start..end)
    }
}

fn patch_asm_bytes() -> &'static [u8] {
    unsafe {
        let start = &raw const patch_start;
        let end = start.with_addr((&raw const patch_end).addr());
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
