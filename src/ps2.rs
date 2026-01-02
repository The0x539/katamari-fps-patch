use std::sync::Once;

use windows::Win32::Foundation::HMODULE;

use crate::types::{Camera, Katamari, Mat4, Prince, Vec4};

#[macro_use]
mod macros;

mod nil;
use nil::Nil;

pub static mut DLL: PS2 = PS2::NIL;

pub unsafe fn link(module: HMODULE) {
    static INIT: Once = Once::new();
    INIT.call_once(|| unsafe {
        DLL = PS2::from_module(module);
    });
}

exports! {
    struct PS2;

    #[func @ 0x1e6a0]
    fn katamari_physics_big_kahuna(k: *mut Katamari);

    #[func @ 0x59f20]
    fn normalize(dst: *mut Vec4, src: *const Vec4);

    #[func @ 0x25fb0]
    fn x25fb0(v: *const Vec4);

    #[func @ 0x21e50]
    fn climb_guy(_v: *const Vec4, k: *mut Katamari);

    #[func @ 0x12ca0]
    fn terminate_climb(k: *mut Katamari);

    #[func @ 0x1ee70]
    fn update_katamari_measurements(k: *mut Katamari);

    #[func @ 0x1ff50]
    fn katamari_physics_sub_sub1(k: *mut Katamari, v: *const Vec4);

    #[func @ 0x1b3b0]
    fn katamari_physics_prop_collision(k: *mut Katamari);

    #[func @ 0x20660]
    fn x20660(k: *mut Katamari);

    #[func @ 0x26b80]
    fn apply_deadzones(out: *mut Vec4, v: *const Vec4, threshold: f32);

    #[func @ 0x23b70]
    fn gravity_user_3(k: *mut Katamari);

    #[func @ 0xad40]
    fn set_player_animation_mode(p_idx: i32, mode: u8);

    #[func @ 0x1db50]
    fn speed_thing_2(k: *mut Katamari, k_: *mut Katamari);

    #[func @ 0x21590]
    fn calculate_friction(k: *mut Katamari);

    #[func @ 0x20cd0]
    fn calculate_gravity_and_some_other_forces(k: *mut Katamari);

    #[func @ 0x59590]
    fn copy_matrix(dst: *mut Mat4, src: *const Mat4) -> *mut Mat4;

    #[func @ 0x56510]
    fn handle_turn(prince: *mut Prince);

    #[func @ 0x55b70]
    fn actually_apply_player_input_force_2(prince: *mut Prince);

    #[func @ 0x56250]
    fn gentle_steering(prince: *mut Prince);

    #[func @ 0x5afc0]
    fn ontick_update_angle_guy();

    #[func @ 0x533d0]
    fn update_prince();

    #[func @ 0x53650]
    fn update_prince_position(prince: *mut Prince, v: *mut Vec4);

    #[func @ 0x55480]
    fn prince_flip(prince: *mut Prince);

    #[func @ 0x566d0]
    fn prince_handle_dash(p_idx: i32, prince: *mut Prince);

    #[func @ 0x25be0]
    fn tick_player(p_idx: i32, prince: *mut Prince);

    #[var @ 0x07b218]
    static val_x7b218: f32;

    #[var @ 0x07b0ec]
    static g_katamari_speed_f: f32;

    #[var @ 0x07acf4]
    static g_katamari_rot: f32;

    #[var @ 0x0ff0f5]
    static multiplayer: bool;

    #[var @ 0x10eb18]
    static climb_limit: i32;

    #[var @ 0x10daf5]
    static game_mode: u8;

    #[var @ 0x10eae4]
    static delta_time: f32;

    #[var @ 0x0ff108]
    static current_stage: u8;

    #[var @ 0x10daed]
    static val_x10daed: bool; // related to the "NPC approaching" sound

    #[var @ 0x0ff0f6]
    static val_x0ff0f6: u8; // related to the "NPC approaching" sound

    #[var @ 0x0ff0f4]
    static current_player_index: u8;

    #[var @ 0x07b1a0]
    static dash_input_window: i32;

    #[array @ 0xd33210]
    static prince_array: [Prince; 2];

    #[array @ 0x192ee0]
    static camera_array: [Camera; 2];

    #[array @ 0x16d720]
    static katamari_array: [Katamari; 2];

    #[callback @ 0x10ea00]
    fn play_visual_fx(
        vfx_id: i32,
        pos_x: f32,
        pos_y: f32,
        pos_z: f32,
        dir_x: f32,
        dir_y: f32,
        dir_z: f32,
        scale: f32,
        attach_id: i32,
        player_id: i32,
    );

    #[callback @ 0x10ea18]
    fn play_sound_fx(sfx_id: i32, volume: f32, pan: i32);
}
