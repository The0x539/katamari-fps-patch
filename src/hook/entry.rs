use windows::Win32::Foundation::HMODULE;

// I have this stashed in like three other places, but whatever
#[unsafe(no_mangle)]
static mut OTHER_DLL_BASE: *mut u8 = std::ptr::null_mut();

#[unsafe(no_mangle)]
static mut TARGETS: [*mut Page; PAGE_COUNT] = [std::ptr::null_mut(); PAGE_COUNT];

type Page = [*const (); 0x10000];

pub(super) static mut TRAMPOLINE_PHASE1: *mut u8 = std::ptr::null_mut();

#[link(name = "native_replacements", kind = "static")]
unsafe extern "C" {
    fn trampoline_phase2();
}

const PAGE_COUNT: usize = 0x60;

pub unsafe fn init(dll: HMODULE) {
    let mut trampoline_code = [0xcc; 0x20];

    // The memory 0xA bytes after the end of this instruction contains an absolute address.
    // Jump to that address.
    trampoline_code[..6].copy_from_slice(&[0xFF, 0b00_100_101, /**/ 0xA, 0, 0, 0]);

    // In the aforementioned memory, place a pointer to the trampoline_phase2 function.
    let phase2_ptr = trampoline_phase2 as *const () as usize;
    trampoline_code[0x10..0x18].copy_from_slice(&phase2_ptr.to_ne_bytes());

    unsafe {
        OTHER_DLL_BASE = dll.0.cast();
        // Insert our stuff at the end of PS2KatamariSimulation.text, shortly after its final functions
        TRAMPOLINE_PHASE1 = OTHER_DLL_BASE.add(0x5ec50);

        // Check that we're not overwriting anything obviously important.
        // (This is *expected* to be unused space at the end of the code segment.)
        for i in 0..trampoline_code.len() {
            assert_eq!(*TRAMPOLINE_PHASE1.byte_add(i), 0);
        }

        super::raw_patch(TRAMPOLINE_PHASE1, 0, trampoline_code).unwrap();
    }
}

pub unsafe fn register(origin: *const u8, target: *const ()) {
    unsafe {
        let origin = origin.addr();
        let base = OTHER_DLL_BASE.addr();
        assert!(origin >= base);
        let offset = origin - base;

        let idx_outer = offset >> 16;
        let idx_inner = offset & 0xFFFF;

        // This should be enough of a range to address all of the DLL's code segment.
        assert!(idx_outer < PAGE_COUNT);

        let page: *mut *mut Page = &raw mut TARGETS[idx_outer];
        if (*page).is_null() {
            *page = calloc::<Page>();
        }

        let entry: *mut *const () = &raw mut (**page)[idx_inner];

        if !(*entry).is_null() {
            println!("ERROR: Duplicate hook entry for code offset {offset:#x}");
            return;
        }

        *entry = target;
    }
}

unsafe fn calloc<T>() -> *mut T {
    unsafe {
        let layout = std::alloc::Layout::new::<T>();
        std::alloc::alloc_zeroed(layout).cast::<T>()
    }
}
