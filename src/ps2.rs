use std::sync::Once;

use windows::Win32::Foundation::HMODULE;

use crate::types::{Camera, Katamari, Prince, Vec4};

type KFn = extern "win64" fn(*mut Katamari);

#[derive(Debug, Copy, Clone)]
pub struct PS2 {
    pub katamari_physics_big_kahuna: KFn,
    pub normalize: extern "win64" fn(*mut Vec4, *const Vec4),
    pub x25fb0: extern "win64" fn(*const Vec4),
    pub climb_guy: extern "win64" fn(*const Vec4, *mut Katamari),
    pub terminate_climb: KFn,
    pub update_katamari_measurements: KFn,
    pub katamari_physics_sub_sub1: extern "win64" fn(*mut Katamari, *const Vec4),
    pub x20660: KFn,
    pub katamari_physics_big_subroutine: KFn,
    pub apply_deadzones: extern "win64" fn(*mut Vec4, *const Vec4, f32),
    pub gravity_user_3: KFn,
    pub set_player_animation_mode: extern "win64" fn(i32, u8),

    pub multiplayer: *mut bool,
    pub climb_limit: *mut i32,
    pub g_katamari_speed_f: *mut f32,
    pub game_mode: *mut u8,
    pub val_x7b218: *mut f32,
    pub current_stage: *mut u8,
    pub delta_time: *mut f32,

    pub prince_array: *mut [Prince; 2],
    pub camera_array: *mut [Camera; 2],
    pub katamari_array: *mut [Katamari; 2],

    pub cb_play_visual_fx: *mut extern "win64" fn(
        vfx_id: i32,
        pos: f32,
        f32,
        f32,
        dir: f32,
        f32,
        f32,
        scale: f32,
        attach_id: i32,
        player_id: i32,
    ),
}

impl PS2 {
    const fn nil() -> Self {
        extern "win64" fn nil<T>(_x: T) {}
        extern "win64" fn nil2<T, U>(_x: T, _y: U) {}
        extern "win64" fn nil3<T, U, V>(_x: T, _y: U, _z: V) {}

        use std::ptr::null_mut;
        Self {
            katamari_physics_big_kahuna: nil,
            normalize: nil2,
            x25fb0: nil,
            climb_guy: nil2,
            terminate_climb: nil,
            update_katamari_measurements: nil,
            katamari_physics_sub_sub1: nil2,
            x20660: nil,
            katamari_physics_big_subroutine: nil,
            apply_deadzones: nil3,
            gravity_user_3: nil,
            set_player_animation_mode: nil2,

            multiplayer: null_mut(),
            climb_limit: null_mut(),
            g_katamari_speed_f: null_mut(),
            game_mode: null_mut(),
            val_x7b218: null_mut(),
            current_stage: null_mut(),
            delta_time: null_mut(),

            prince_array: null_mut(),
            camera_array: null_mut(),
            katamari_array: null_mut(),

            cb_play_visual_fx: null_mut(),
        }
    }

    unsafe fn from_module(module: HMODULE) -> Self {
        unsafe {
            let base = module.0;
            macro_rules! func {
                ($offset:literal) => {
                    std::mem::transmute(base.offset($offset))
                };
            }
            macro_rules! ptr {
                ($offset:literal) => {
                    base.offset($offset).cast()
                };
            }

            Self {
                katamari_physics_big_kahuna: func!(0x1e6a0),
                normalize: func!(0x59f20),
                x25fb0: func!(0x25fb0),
                climb_guy: func!(0x21e50),
                terminate_climb: func!(0x12ca0),
                update_katamari_measurements: func!(0x1ee70),
                katamari_physics_sub_sub1: func!(0x1ff50),
                x20660: func!(0x20660),
                katamari_physics_big_subroutine: func!(0x1b3b0),
                apply_deadzones: func!(0x26b80),
                gravity_user_3: func!(0x23b70),
                set_player_animation_mode: func!(0xad40),

                val_x7b218: ptr!(0x07b218),
                g_katamari_speed_f: ptr!(0x07b0ec),
                multiplayer: ptr!(0x0ff0f5),
                climb_limit: ptr!(0x10eb18),
                game_mode: ptr!(0x10daf5),
                delta_time: ptr!(0x10eae4),
                current_stage: ptr!(0xff108),

                prince_array: ptr!(0xd33210),
                camera_array: ptr!(0x192ee0),
                katamari_array: ptr!(0x16d720),

                cb_play_visual_fx: ptr!(0x10ea00),
            }
        }
    }
}

pub static mut DLL: PS2 = PS2::nil();

pub unsafe fn link(module: HMODULE) {
    static INIT: Once = Once::new();
    INIT.call_once(|| unsafe {
        DLL = PS2::from_module(module);
        println!("actually did the guy");
    });
}

macro_rules! export_functions {
    ($(
        fn $func:ident(
            $($arg:ident : $aty:ty),*$(,)?
        ) $(-> $ret:ty)? ;
    )*) => {$(
        pub fn $func( $($arg : $aty),* ) $(-> $ret)? {
            unsafe { (DLL.$func)($($arg),*) }
        }
    )*}
}

export_functions! {
    fn normalize(dst: *mut Vec4, src: *const Vec4);
    fn x25fb0(v: *const Vec4);
    fn climb_guy(_v: *const Vec4, k: *mut Katamari);
    fn terminate_climb(k: *mut Katamari);
    fn update_katamari_measurements(k: *mut Katamari);
    fn katamari_physics_sub_sub1(k: *mut Katamari, v: *const Vec4);
    fn katamari_physics_big_subroutine(k: *mut Katamari);
    fn x20660(k: *mut Katamari);
    fn apply_deadzones(out: *mut Vec4, v: *const Vec4, threshold: f32);
    fn gravity_user_3(k: *mut Katamari);
    fn set_player_animation_mode(p_idx: i32, mode: u8);
}

// TODO: move struct code into here and make an initializer using the offset
macro_rules! export_variables {
    ($(
        static $var:ident : $ty:ty $(= * $offset:literal)? ;
    )*) => {$(
        pub fn $var() -> $ty {
            unsafe { *DLL.$var }
        }
    )*}
}

export_variables! {
    static multiplayer: bool;
    static climb_limit: i32;
    static g_katamari_speed_f: f32;
    static game_mode: u8;
    static val_x7b218: f32;
    static current_stage: u8;
    static delta_time: f32;
}

#[inline]
pub fn prince_ptr(player_index: impl Into<i64>) -> *mut Prince {
    let i = player_index.into() as usize;
    unsafe { &raw mut (*DLL.prince_array)[i] }
}

#[inline]
pub fn camera_ptr(player_index: impl Into<i64>) -> *mut Camera {
    let i = player_index.into() as usize;
    unsafe { &raw mut (*DLL.camera_array)[i] }
}

#[inline]
pub fn katamari_ptr(player_index: impl Into<i64>) -> *mut Katamari {
    let i = player_index.into() as usize;
    unsafe { &raw mut (*DLL.katamari_array)[i] }
}

pub mod cb {
    use crate::types::Vec4;

    use super::DLL;

    pub fn play_visual_fx(
        vfx_id: i32,
        pos: &Vec4,
        dir: &Vec4,
        scale: f32,
        attach_id: i32,
        player_id: i32,
    ) {
        unsafe {
            (*DLL.cb_play_visual_fx)(
                vfx_id, pos.x, pos.y, pos.z, dir.x, dir.y, dir.z, scale, attach_id, player_id,
            )
        }
    }
}
