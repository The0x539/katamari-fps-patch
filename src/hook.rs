use crate::w;

pub mod entry;

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

pub unsafe fn patch(
    target: impl Target,
    range: std::ops::Range<usize>,
    detour: *const (),
) -> eyre::Result<()> {
    let target = target.into_target();
    unsafe {
        let target_code = std::ptr::slice_from_raw_parts_mut(target.add(range.start), range.len());

        let patch_code = generate_call(target_code.cast(), entry::TRAMPOLINE_PHASE1);
        entry::register(target_code.get_unchecked_mut(patch_code.len()), detour);

        mprotect(target_code, w::PAGE_READWRITE, || {
            let target_code = &mut *target_code;
            target_code[..patch_code.len()].copy_from_slice(&patch_code);
            fill_with_nops(&mut target_code[patch_code.len()..]);
        })?;
    }

    Ok(())
}

unsafe fn generate_call(call_addr: *const u8, target_addr: *const u8) -> [u8; 5] {
    // This mechanism relies on the two addresses being nearby enough for 32-bit relative addressing.
    // Originally, this mod just assumed both relevant DLLs would be loaded within a suitable range.
    //
    // This assumption was semi-reliable, but would sometimes break upon recompiling mods
    // or when several other programs were open (complicating the shared library address space).
    //
    // Now we only use this to call a single small trampoline function,
    // which lives inside the patched DLL so as to always be within rel32 range.
    // "Phase 1" of the trampoline performs an absolute jump to "phase 2".
    // Phase 2 uses the return address as a key to look up which actual-target function to jump to.
    let rel32 = unsafe {
        let return_addr: *const u8 = call_addr.add(5);
        let rel64: isize = target_addr.byte_offset_from(return_addr);
        i32::try_from(rel64).unwrap_or_else(|_| {
            println!("could not make rel32 address");
            std::thread::sleep(std::time::Duration::from_secs(5));
            std::process::exit(1)
        })
    };
    let mut code = [0_u8; 5];
    code[0] = 0xE8; // CALL rel32
    code[1..].copy_from_slice(&rel32.to_ne_bytes());
    code
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
            fill_with_nops(&mut *target_code);
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

unsafe impl Target for *mut u8 {
    fn into_target(self) -> *mut u8 {
        self
    }
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

unsafe impl<T, U, V, W, R> Target for extern "win64" fn(T, U, V, W) -> R {
    fn into_target(self) -> *mut u8 {
        self as _
    }
}

unsafe impl<T, U, V, W, X, R> Target for extern "win64" fn(T, U, V, W, X) -> R {
    fn into_target(self) -> *mut u8 {
        self as _
    }
}

unsafe extern "C" {
    static hook_start: u8;
    static hook_end: u8;
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

fn fill_with_nops(code: &mut [u8]) {
    for chunk in code.chunks_mut(9) {
        let nop: &[u8] = match chunk.len() {
            0 => &[],
            1 => &[0x90],
            2 => &[0x66, 0x90],
            3 => &[0x0f, 0x1f, 0],
            4 => &[0x0f, 0x1f, 0x40, 0x00],
            5 => &[0x0f, 0x1f, 0x44, 0x00, 0x00],
            6 => &[0x66, 0x0f, 0x1f, 0x44, 0x00, 0x00],
            7 => &[0x0f, 0x1f, 0x80, 0x00, 0x00, 0x00, 0x00],
            8 => &[0x0f, 0x1f, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00],
            9 => &[0x66, 0x0f, 0x1f, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00],
            10.. => unreachable!(),
        };
        chunk.copy_from_slice(nop);
    }
}
