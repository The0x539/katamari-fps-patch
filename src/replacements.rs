use std::arch::{asm, naked_asm};
use std::f32::consts::{PI, TAU};

use crate::ps2;
use crate::types::*;

pub mod cacophony;
pub mod dash;
pub mod experiments;
pub mod movement;

pub use cacophony::*;
pub use dash::*;
pub use experiments::*;
pub use movement::*;

pub(crate) mod values {
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

    /// The duration, in *ticks*, of the current update.
    ///
    /// If a value is measured in "units per tick", e.g. velocity,
    /// it should probably be multiplied by this value.
    pub static mut DT_TICKS: f32 = 0.5;

    /// The duration, in (rounded) *milliseconds*, of the current update.
    ///
    /// If the original code uses an integer to count ticks,
    /// then updating it to instead count milliseconds
    /// will require using this value (instead of 1) as an increment/decrement.
    pub static mut DT_MILLIS: i16 = 16;

    /// The accumulated rounding error of DT_MILLIS.
    static mut DT_MICROS: i16 = 0;

    pub fn set_dt(delta: f32) {
        unsafe {
            DT_SECONDS = delta;
            DT_TICKS = delta * 1000.0 / 30.0;

            let millis = delta * 1000.0;
            DT_MILLIS = millis.floor() as i16;

            let micros = (millis - millis.floor()) * 1000.0;
            DT_MICROS += micros.round() as i16;
            while DT_MICROS > 1000 {
                DT_MILLIS += 1;
                DT_MICROS -= 1000;
            }
        }
    }

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

    pub static mut MULTIPLAYER: u8 = 0;
    // TODO: add more stuff here as necessary and update it whenever needed

    // Exposed for convenience from safe-Rust code.
    #[inline]
    pub(super) fn dt_millis() -> i16 {
        unsafe { DT_MILLIS }
    }

    #[inline]
    pub(super) fn dt_ticks() -> f32 {
        unsafe { DT_TICKS }
    }

    #[inline]
    pub(super) fn multiplayer() -> bool {
        unsafe { MULTIPLAYER != 0 }
    }
}

// Shelved for now. Harder to do than I expected.
pub unsafe extern "C" fn prince_flip() {
    unsafe {
        let prince: *mut Prince;
        asm!("", out("rdi") prince);

        let mut yaw = (*prince).actual_yaw;
        if yaw.is_nan() {
            yaw = 0.0;
        }
        // TODO: I'm not sure this is actually ever used properly,
        // but it might have something to do with the joysticks
        let foo = (*prince).prince_flip_rate_guy;
        if foo != 0.0 {
            println!("{foo}");
        }
        yaw += foo * values::DT_TICKS;
        if yaw > PI {
            yaw -= TAU;
        }
        if yaw < -PI {
            yaw += TAU;
        }
        (*prince).actual_yaw = yaw;

        // this seems to be the only part of register state
        // that the function touches in the replaced block and also uses later
        asm! {
            "xor rbx, rbx",
            "mov rcx, rbx",
        }
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
        }
    }
}

pub unsafe extern "C" fn copy_matrix(dst: *mut Mat4, src: *const Mat4) -> *mut Mat4 {
    unsafe {
        // compiles to two ymmword load/store pairs
        // TODO: make sure clobbering ymm0/ymm1 is okay for the caller...
        *dst = *src;
        dst
    }
}
