use std::sync::Once;

use windows::Win32::Foundation::HMODULE;

use crate::types::{Camera, CameraTransform, Katamari, Mat4, Prince, PropConstants, Thing, Vec4};

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
    fn katamari_physics_climb(_v: *const Vec4, k: *mut Katamari);

    #[func @ 0x12ca0]
    fn terminate_climb(k: *mut Katamari);

    #[func @ 0x1ee70]
    fn update_katamari_measurements(k: *mut Katamari);

    #[func @ 0x1db50]
    fn do_katamari_physics(k: *mut Katamari, delta: f32);

    #[func @ 0x1ff50]
    fn katamari_update_spin(k: *mut Katamari, v: *const Vec4);

    #[func @ 0x1b3b0]
    fn katamari_pivot(k: *mut Katamari);

    #[func @ 0x20660]
    fn katamari_physics_roll(k: *mut Katamari);

    #[func @ 0x26b80]
    fn apply_deadzones(out: *mut Vec4, v: *const Vec4, threshold: f32);

    #[func @ 0x23b70]
    fn gravity_user_3(k: *mut Katamari);

    #[func @ 0x22130]
    fn speed_thing_2(k: *mut Katamari, k_: *mut Katamari);

    #[func @ 0x21590]
    fn calculate_friction(k: *mut Katamari);

    #[func @ 0x20cd0]
    fn calculate_gravity(k: *mut Katamari);

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

    #[func @ 0x52bd0]
    fn initialize_princes();

    #[func @ 0x56e60]
    fn prince_exhausted(p_idx: i32, prince: *mut Prince);

    #[func @ 0x56650]
    fn prince_reset_exhaustion(prince: *mut Prince);

    #[func @ 0x178e0]
    fn splash(_rcx: *mut (), k: *mut Katamari);

    #[func @ 0x07080]
    fn gunshot(_m: *mut ());

    #[func @ 0x1c530]
    fn some_sort_of_collision_guy(k: *mut Katamari);

    #[func @ 0x5a970]
    fn rotate_about_axis(m: *mut Mat4, v: *const Vec4, theta: f32);

    #[func @ 0x1c8e0]
    fn x1c8e0(k: *mut Katamari);

    #[func @ 0x20b00]
    fn x20b00(k: *mut Katamari);

    #[func @ 0x40bb0]
    fn npc_40bb0(p: *mut Thing);

    #[func @ 0x31a30]
    fn thing_animal_motion(p: *mut Thing, animal: *mut (), flag: bool);

    #[func @ 0x5a970]
    fn rotation_from_axis_angle(out: *mut Mat4, axis: *const Vec4, angle: f32);

    #[func @ 0xebf0]
    fn camera_bear_cow_orbit(p_idx: i32, cam: *mut Camera);

    #[func @ 0xef70]
    fn camera_versus_winner_orbit(p_idx: i32, cam: *mut Camera);

    #[func @ 0xb7d0]
    fn camera_animate();

    #[func @ 0xbe60]
    fn camera_update_katamari_view(p_idx: u64, cam: *mut Camera, k: *mut Katamari);

    #[func @ 0xad40]
    fn camera_set_view_mode(p_idx: i32, mode: u8);

    #[func @ 0x28870]
    fn katamari_queue_things_for_pickup(k: *mut Katamari);

    #[func @ 0x24460]
    fn katamari_sink_things();

    #[func @ 0x44bb0]
    fn thing_reset_hop_timer(t: *mut Thing);

    #[func @ 0x44c00]
    fn thing_hop(t: *mut Thing);

    #[func @ 0x45180]
    fn thing_hop_turn(t: *mut Thing);

    #[func @ 0x320f0]
    fn thing_hop_apply_velocity(t: *mut Thing);

    #[func @ 0x2af40]
    fn katamari_bump_thing(k: *mut Katamari, t: *mut Thing);

    #[func @ 0x2eb10]
    fn thing_freefall(t: *mut Thing);

    #[func @ 0x367e0]
    fn thing_random_hop_main(t: *mut Thing);

    #[func @ 0x3a8f0]
    fn f32_angle_move_towards(angle: *mut f32, delta: *const f32, target: *const f32);

    #[func @ 0x3e040]
    fn pursuit_angle_move_towards(t: *mut Thing, pursuit_a: *mut (), angle: f32, flag: bool);

    #[func @ 0x3ad20]
    fn thing_animal_state_0_start(t: *mut Thing);

    #[func @ 0x3b1e0]
    fn thing_animal_state_1_walk(t: *mut Thing);

    #[func @ 0x3bcd0]
    fn thing_animal_state_3_turn(t: *mut Thing);

    #[func @ 0x3bfb0]
    fn thing_animal_x3bfb0(t: *mut Thing);

    #[func @ 0x391b0]
    fn thing_train_x391b0(t: *mut Thing);

    #[func @ 0xc500]
    fn camera_update_xc500(p_idx: i64, cam: *mut Camera, k: *mut Katamari, prince: *const Prince, ordinary: bool);

    #[func @ 0x39e40]
    fn thing_flee_state_1(t: *mut Thing);

    #[func @ 0x39ee0]
    fn thing_flee_state_2(t: *mut Thing);

    #[func @ 0x3a270]
    fn thing_flee_state_4(t: *mut Thing);

    #[func @ 0x3c8e0]
    fn thing_pursuit_alt_state_1(t: *mut Thing);

    #[func @ 0x3c950]
    fn thing_pursuit_alt_state_2(t: *mut Thing);

    #[func @ 0x28ef0]
    fn katamari_attach_thing_x28ef0(k: *mut Katamari, t: *mut Thing);

    #[func @ 0x15950]
    fn katamari_collide_with_wall(_rcx: usize, k: *mut Katamari);

    #[func @ 0x3f510]
    fn thing_machine_22_state_2(t: *mut Thing);

    #[func @ 0x3f590]
    fn thing_machine_22_state_3(t: *mut Thing);

    #[func @ 0x3fd30]
    fn thing_machine_22_state_6(t: *mut Thing);

    #[func @ 0x40a60]
    fn thing_machine_20_state_1(t: *mut Thing);

    #[func @ 0x2f100]
    fn thing_basic_gravity(t: *mut Thing);

    #[func @ 0x25590]
    fn camera_main();

    #[func @ 0x32cc0]
    fn thing_wobble_state_1(t: *mut Thing);

    #[func @ 0x36730]
    fn thing_start_rng_timer(t: *mut Thing);

    #[func @ 0x37610]
    fn thing_check_reached_target_path_node(t: *mut Thing);

    #[func @ 0x36a10]
    fn thing_x36a10(t: *mut Thing);

    #[func @ 0x35230]
    fn thing_getup_flee_state_0(t: *mut Thing, flee: *mut ());

    #[func @ 0x35270]
    fn thing_getup_flee_state_1(t: *mut Thing, flee: *mut ());

    #[func @ 0x352e0]
    fn thing_getup_flee_state_2(t: *mut Thing, flee: *mut ());

    #[func @ 0x356a0]
    fn thing_getup_flee_state_3a(t: *mut Thing, flee: *mut ());

    #[func @ 0x35c10]
    fn thing_getup_flee_state_3b(t: *mut Thing, flee: *mut ());

    #[func @ 0x356e0]
    fn thing_getup_flee_state_4a(t: *mut Thing, flee: *mut ());

    #[func @ 0x35c80]
    fn thing_getup_flee_state_4b(t: *mut Thing, flee: *mut ());

    #[func @ 0x2b420]
    fn katamari_flip_thing(k: *mut Katamari, t: *mut Thing);

    #[func @ 0x27170]
    fn x27170(v: *mut Vec4, k: *mut Katamari, t: *mut Thing, n: f32);

    #[func @ 0x29480]
    fn katamari_hit_test(k: *mut Katamari, t: *mut Thing) -> i32;

    #[func @ 0x28e00]
    fn katamari_x28e00(k: *mut Katamari, t: *mut Thing);

    #[func @ 0x49d40]
    fn thing_spin_x49d40(t: *mut Thing);

    #[func @ 0x39900]
    fn thing_windmill_spin_x39900(_a: *mut (), _b: *mut (), theta: *mut f32);

    #[func @ 0x379a0]
    fn thing_walk_turn(machine: *mut ());

    #[func @ 0x4a050]
    fn thing_pendulum(t: *mut Thing);

    #[func @ 0x48490]
    fn thing_wrecking_ball_pendulum(t: *mut Thing);

    #[func @ 0x428d0]
    fn thing_bird_state_0_init(t: *mut Thing);

    #[func @ 0x42950]
    fn thing_bird_state_1_idle(t: *mut Thing);

    #[func @ 0x42da0]
    fn thing_bird_state_4_ascend(t: *mut Thing);

    #[func @ 0x42ee0]
    fn thing_bird_state_5_stay_in_sky(t: *mut Thing);

    #[func @ 0x42f50]
    fn thing_bird_state_6_begin_descent(t: *mut Thing);

    #[func @ 0x43100]
    fn thing_bird_state_7_descend(t: *mut Thing);

    #[func @ 0x46a20]
    fn thing_scarecrow_sway(t: *mut Thing);

    #[func @ 0x54160]
    fn prince_read_sticks(p: *mut Prince);

    #[func @ 0x54e90]
    fn prince_forced_turn(p: *mut Prince);

    #[func @ 0x31f90]
    fn thing_animal_x31f90(t: *mut Thing, animal: *mut ());

    #[func @ 0x7610]
    fn camera_credits_update();

    #[func @ 0x2b10]
    fn init();

    #[func @ 0x4fd0]
    fn mono_init_start(mono_data: *mut (), mission: i32, area: i32, stage: u8, kadai_flag: bool, clear_flag: bool, end_flag: bool);

    #[func @ 0x30490]
    fn thing_calc_velocity(t: *mut Thing, v: *const Vec4);

    #[func @ 0x28640]
    fn katamari_x28640(k: *mut Katamari);

    #[func @ 0x2f8e0]
    fn thing_collide_with_other_thing(t: *mut Thing, idx: u16);

    #[func @ 0x14c80]
    fn katamari_physics_x14c80(rcx: u64, k: *mut Katamari);

    #[func @ 0x4a9f0]
    fn thing_kickball_state_1(t: *mut Thing);

    #[func @ 0x4b1d0]
    fn thing_kickball_physics(t: *mut Thing);

    #[func @ 0x39440]
    fn thing_walk_cycle_x39440(t: *mut Thing);

    #[func @ 0x41ce0]
    fn thing_walk_cycle_x41ce0(t: *mut Thing);

    #[func @ 0x42080]
    fn thing_walk_cycle_x42080(t: *mut Thing);

    #[func @ 0x46250]
    fn thing_golfer_animation_state_0(t: *mut Thing);

    #[func @ 0x46280]
    fn thing_golfer_animation_state_1(t: *mut Thing);

    #[func @ 0x462a0]
    fn thing_golfer_animation_state_2(t: *mut Thing);

    #[func @ 0x462c0]
    fn thing_golfer_animation_state_3(t: *mut Thing);

    #[func @ 0x54c90]
    fn prince_first_person_controls(p: *mut Prince);

    #[var @ 0]
    static base_addr: ();

    #[var @ 0x07b218]
    static val_x7b218: f32;

    #[var @ 0x07a25c]
    static g_katamari_speed_d: f32;

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
    static allow_sfx: bool;

    #[var @ 0x0ff0f6]
    static val_x0ff0f6: u8; // related to the "NPC approaching" sound

    #[var @ 0x10ea50]
    static tick_count: u32;

    #[var @ 0x0ff0f4]
    static current_player_index: u8;

    #[var @ 0x07b1a0]
    static dash_input_window: i32;

    #[var @ 0x10e084]
    static gravity: Vec4;

    #[var @ 0xb23e0]
    static camera_smoothing: Vec4;

    #[var @ 0x10eb18]
    static climb_sustain_limit: u32;

    #[var @ 0x15523c]
    static thing_gravity: f32;

    #[var @ 0xd34c28]
    static angel_zoom_distance: f32;

    #[var @ 0xd34c2c]
    static angel_zoom_rate: f32;

    #[var @ 0x7bc44]
    static rng: i32;

    // accessed as i16 by the game but there's padding that lets me use i32 instead
    #[var @ 0xd34c38]
    static credits_timer: i32;

    #[var @ 0x7b25c]
    static air_time_min: i32; // normally 5 frames

    #[var @ 0x7b260]
    static fall_time_min: i32; // normally 9 frames

    #[var @ 0x7b264]
    static fall_time_max: i32; // normally 46 frames

    #[array @ 0xd33210]
    static prince_array: [Prince; 2];

    #[array @ 0x192ee0]
    static camera_array: [Camera; 2];

    #[array @ 0x16d720]
    static katamari_array: [Katamari; 2];

    #[array @ 0xd34180]
    static camera_transform_array: [CameraTransform; 2];

    #[array @ 0x19b010]
    static prop_array: [Thing; 4000];

    #[array @ 0x8a880]
    static prop_data_array: [PropConstants; 1718];

    #[array @ 0x7a040]
    static bump_timers: [u16; 2];

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

pub fn play_visual_fx(
    vfx_id: i32,
    position: Vec4,
    direction: Vec4,
    scale: f32,
    attach_id: i32,
    player_id: i32,
) {
    cb::play_visual_fx(
        vfx_id,
        position.x,
        position.y,
        position.z,
        direction.x,
        direction.y,
        direction.z,
        scale,
        attach_id,
        player_id,
    );
}

// SAFETY: Beware of aliasing.
// Other functions may get their own pointer to the same object,
// in which case the compiler may make incorrect assumptions about what happens to the fields.
// I don't expect this to be an actual problem in practice,
// since I'm looking so closely at the compiler output anyway.
pub unsafe fn current_katamari<'a>() -> &'a mut Katamari {
    unsafe { &mut *katamari_array(current_player_index()) }
}

// See above.
pub unsafe fn current_prince<'a>() -> &'a mut Prince {
    unsafe { &mut *prince_array(current_player_index()) }
}

// See above.
pub unsafe fn current_camera<'a>() -> &'a mut Camera {
    unsafe { &mut *camera_array(current_player_index()) }
}
