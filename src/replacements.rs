use std::arch::{asm, naked_asm};
use std::f32::consts::PI;

use crate::ps2;
use crate::types::*;

pub mod abilities;
pub mod cacophony;
pub mod camera;
pub mod dash;
pub mod movement;
pub mod stereo_haptics;
pub mod things;

pub(crate) mod values {
    use crate::types::Vec4;

    /// Let a "tick" refer to a 1/30 second duration.
    /// Let an "update" refer to one call of the Tick() function from PS2KatamariSimulation.dll.
    ///
    /// The vanilla game assumes that each update simulates one tick.
    /// Changing this requires a great deal of x86 surgery.
    ///
    /// Mutable variables in this module will be updated once each tick.
    /// Doing this ahead of time, and having these variables within this DLL's address space,
    /// allows the hooks' inline assembly to use the values with fewer instructions and fewer registers.
    ///
    /// (The initial values are not expected to be observed, but are based on having 60 updates per second.)

    /// The duration, in *seconds*, of the current update.
    ///
    /// Corresponds directly to Unity's `time.deltaTime`,
    /// or at least will when the C# side of this mod is complete.
    pub static mut DT_SECONDS: f32 = 1.0 / 60.0;

    #[repr(align(16))]
    pub struct AlignedVec4(pub Vec4);

    /// The duration, in *ticks*, of the current update.
    ///
    /// If a value is measured in "units per tick", e.g. velocity,
    /// it should probably be multiplied by this value.
    ///
    /// This value is four
    #[unsafe(no_mangle)]
    pub static mut DT_TICKS: AlignedVec4 = AlignedVec4(Vec4::splat3(0.5));

    /// The duration, in (rounded) *milliseconds*, of the current update.
    ///
    /// If the original code uses an integer to count ticks,
    /// then updating it to instead count milliseconds
    /// will require using this value (instead of 1) as an increment/decrement.
    ///
    /// This is never expected to overflow u8, let alone i16,
    /// but using 32 bits is helpful for some asm.
    #[unsafe(no_mangle)]
    pub static mut DT_MILLIS: i32 = 16;

    /// The accumulated rounding error of DT_MILLIS.
    static mut DT_MICROS: i16 = 0;

    #[unsafe(no_mangle)]
    static mut KICKBALL_DECEL: f32 = 0.98;

    pub fn set_dt(delta: f32) {
        unsafe {
            DT_SECONDS = delta;
            let dt_ticks = delta * 30.0;
            DT_TICKS = AlignedVec4(Vec4::splat3(dt_ticks));

            let millis = delta * 1000.0;
            DT_MILLIS = millis.floor() as i32;

            let micros = (millis - millis.floor()) * 1000.0;
            DT_MICROS += micros.round() as i16;
            while DT_MICROS > 1000 {
                DT_MILLIS += 1;
                DT_MICROS -= 1000;
            }

            // This variable isn't actually semantically much of a vector.
            // Its only usage takes the product of all four elements (each in the range 0 < v <= 1).
            // This product is used as the "lerp smoothing" factor for moving the camera
            // towards its "target" position and orientation.
            //
            // Fortunately, the game only updates this when loading an area,
            // and even then it only touches the x/y/z elements, leaving w untouched at 1.0.
            //
            // We can leave x/y/z untouched and put a value in w that results in an appropriate overall product.
            let smoothing = &mut *crate::ps2::raw::camera_smoothing();
            let vanilla_t = smoothing.x * smoothing.y * smoothing.z;
            let desired_t = 1.0 - (1.0 - vanilla_t).powf(dt_ticks);
            smoothing.w = (desired_t / vanilla_t).clamp(0.01, 1.0);

            KICKBALL_DECEL = 0.98_f32.powf(dt_ticks);
        }
    }

    #[unsafe(no_mangle)]
    pub(crate) static mut PTR_THING_GRAVITY: *mut f32 = std::ptr::null_mut();

    #[unsafe(no_mangle)]
    pub(crate) static mut PTR_CREDITS_TIMER: *mut i32 = std::ptr::null_mut();

    /// The duration, in (rounded) milliseconds, of *one tick*.
    ///
    /// If the original code uses an integer to count ticks,
    /// then updating it to instead count milliseconds
    /// will require multiplying the maximum (*i.e.*: final if counting up; initial if counting down) value
    /// by this ratio, preferably via that timer's "reset_value" field during initialization,
    /// or by changing the hardcoded constant that the timer is compared to.
    ///
    /// To reduce rounding errors, prefer *not* to actually use this constant,
    /// instead using the (n * 1000) / 30 order of operations by doing the math inline.
    #[allow(dead_code)]
    pub const TICK_MS: i16 = 1000 / 30;

    #[unsafe(no_mangle)]
    pub static mut MULTIPLAYER: u8 = 0;

    // Exposed for convenience from safe-Rust code.
    #[inline]
    pub(super) fn dt_millis() -> i16 {
        unsafe { DT_MILLIS as i16 }
    }

    #[inline]
    pub(super) fn dt_ticks() -> f32 {
        unsafe { DT_TICKS.0.x }
    }
}

pub unsafe extern "C" fn prince_post_init() {
    for i in 0..=1 {
        let prince = unsafe { &mut *ps2::prince_array(i) };

        // Timer initial/maximum values. Originally counted ticks; will now count milliseconds.
        for val in [
            &mut prince.max_exhaustion,
            &mut prince.stamina_gain_amount,
            &mut prince.stamina_gain_interval,
            &mut prince.stamina_limit,
            &mut prince.dash_input_window,
        ] {
            *val = (*val * 1000) / 30;
        }

        prince.stamina = prince.stamina_limit as i16;

        unsafe {
            values::MULTIPLAYER = ps2::multiplayer() as u8;
            cacophony::GUNSHOT_TIMESTAMP = None;
        }
    }
}

#[link(name = "native_replacements", kind = "static")]
unsafe extern "C" {
    pub fn copy_matrix(dst: *mut Mat4, src: *const Mat4) -> *mut Mat4;
    pub(crate) static copy_matrix_end: u8;
}

pub const fn shuf(x: u8, y: u8, z: u8, w: u8) -> u8 {
    x | (y << 2) | (z << 4) | (w << 6)
}
