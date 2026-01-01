use std::sync::Once;

use windows::Win32::Foundation::HMODULE;

use crate::types::{Camera, Katamari, Mat4, Prince, Vec4};

#[macro_use]
mod macros;

mod nil;
use nil::Nil;

export_functions! {
    DLL_FUNCTIONS: DllFunctions;

    #[addr = 0x1e6a0]
    fn katamari_physics_big_kahuna(k: *mut Katamari);

    #[addr = 0x59f20]
    fn normalize(dst: *mut Vec4, src: *const Vec4);

    #[addr = 0x25fb0]
    fn x25fb0(v: *const Vec4);

    #[addr = 0x21e50]
    fn climb_guy(_v: *const Vec4, k: *mut Katamari);

    #[addr = 0x12ca0]
    fn terminate_climb(k: *mut Katamari);

    #[addr = 0x1ee70]
    fn update_katamari_measurements(k: *mut Katamari);

    #[addr = 0x1ff50]
    fn katamari_physics_sub_sub1(k: *mut Katamari, v: *const Vec4);

    #[addr = 0x1b3b0]
    fn katamari_physics_prop_collision(k: *mut Katamari);

    #[addr = 0x20660]
    fn x20660(k: *mut Katamari);

    #[addr = 0x26b80]
    fn apply_deadzones(out: *mut Vec4, v: *const Vec4, threshold: f32);

    #[addr = 0x23b70]
    fn gravity_user_3(k: *mut Katamari);

    #[addr = 0xad40]
    fn set_player_animation_mode(p_idx: i32, mode: u8);

    #[addr = 0x1db50]
    fn speed_thing_2(k: *mut Katamari, k_: *mut Katamari);

    #[addr = 0x21590]
    fn calculate_friction(k: *mut Katamari);

    #[addr = 0x20cd0]
    fn calculate_gravity_and_some_other_forces(k: *mut Katamari);

    #[addr = 0x59590]
    fn copy_matrix(dst: *mut Mat4, src: *const Mat4) -> *mut Mat4;

    #[addr = 0x56510]
    fn handle_turn(prince: *mut Prince);

    #[addr = 0x55b70]
    fn actually_apply_player_input_force_2(prince: *mut Prince);

    #[addr = 0x56250]
    fn gentle_steering(prince: *mut Prince);
}

export_variables! {
    DLL_VARIABLES: DllVariables;

    #[addr = 0x07b218]
    static val_x7b218: f32;

    #[addr = 0x07b0ec]
    static g_katamari_speed_f: f32;

    #[addr = 0x07acf4]
    static g_katamari_rot: f32;

    #[addr = 0x0ff0f5]
    static multiplayer: bool;

    #[addr = 0x10eb18]
    static climb_limit: i32;

    #[addr = 0x10daf5]
    static game_mode: u8;

    #[addr = 0x10eae4]
    static delta_time: f32;

    #[addr = 0x0ff108]
    static current_stage: u8;
}

#[derive(Debug)]
pub struct PS2 {
    pub functions: DllFunctions,
    pub variables: DllVariables,

    // TODO: these will need their own structs like the above once they scale more
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

impl Nil for PS2 {
    const NIL: Self = Self {
        functions: Nil::NIL,
        variables: Nil::NIL,
        prince_array: Nil::NIL,
        camera_array: Nil::NIL,
        katamari_array: Nil::NIL,
        cb_play_visual_fx: Nil::NIL,
    };
}

pub static mut DLL: PS2 = PS2::NIL;

pub unsafe fn link(module: HMODULE) {
    static INIT: Once = Once::new();
    INIT.call_once(|| unsafe {
        DLL.functions = DllFunctions::from_module(module);
        DLL.variables = DllVariables::from_module(module);

        DLL.prince_array = module.0.offset(0xd33210).cast();
        DLL.camera_array = module.0.offset(0x192ee0).cast();
        DLL.katamari_array = module.0.offset(0x16d720).cast();

        DLL.cb_play_visual_fx = module.0.offset(0x10ea00).cast();
    });
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
