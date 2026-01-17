use std::{
    ops::{Add, AddAssign, Div, Mul, Sub},
    simd::f32x4,
};

#[repr(C)]
pub struct Katamari {
    pub _x0: Q<32>,
    pub player_sub: *mut PlayerSubStruct,
    pub _x28: Q<28>,
    pub player_index: u8,
    pub _x45: Q1,
    pub x46: u8,
    pub _x47: Q1,
    pub _x48: Q4,
    pub _x4c: Q4,
    pub volume: f32,
    pub x54: f32,
    pub _x58: Q<4>,
    pub diameter_cm: f32,
    pub diameter_mm: u32,
    pub base_radius_guy: f32,
    pub radius_cm: f32,
    pub _x6c: Q<4>,
    pub radius_inches: f32,
    pub circumference_cm: f32,
    pub speed: f32,
    pub x7c: f32,
    pub x80: f32,
    pub x84: f32,
    pub x88: f32,
    pub diameter_m: f32,
    pub measurement_guy: f32,
    pub diameter_decimeters: f32,
    pub x98: f32,
    pub x9c: f32,
    pub xa0: f32,
    pub airborne: bool,
    pub climbing: bool,
    pub climb_height_reached: bool,
    pub dwordflag_xa7: [u8; 4],
    pub hit_water: bool,
    pub _xac: Q<7>,
    pub camera_mode_thing: u8,
    pub vfx_x17_flag: bool,
    pub standing_on_prop: bool,
    pub _xb6: Q<3>,
    pub slope_state: u8,
    pub spinning_in_place: bool,
    pub _xbb: Q1,
    pub _xbc: Q1,
    pub _xbd: Q<3>,
    pub xc0: bool,
    pub _xc1: Q1,
    pub xc2: bool,
    pub xc3: u8,
    pub xc4: bool,
    pub xc5: u8,
    pub _xc6: Q<5>,
    pub _xcb: Q4U,
    pub _xcf: Q8U,
    pub _xd7: Q1,
    pub _xd8: Q4,
    pub _xdc: Q<14>,
    pub xea: u8,
    pub xeb: u8,
    pub climb_flag: bool,
    pub speed_limit_check_1: u8,
    pub _xee: Q2,
    pub xf0: bool,
    pub _xf1: Q1,
    pub _xf2: Q2,
    pub _xf4: Q<7>,
    pub xfb: bool,
    pub _xfc: Q<15>,
    pub x10b: bool,
    pub x10c: [u8; 4],
    pub x110: i8,
    pub _x111: Q1,
    pub countdown_x112: i16,
    pub x114: i16,
    pub x116: i16,
    pub countdown_x118_possible_climb: i16,
    pub _x11a: Q<130>,
    pub x19c: f32,
    pub mass: f32,
    pub x1a4: f32,
    pub x1a8: f32,
    pub x1ac: f32,
    pub x1b0: f32,
    pub x1b4: f32,
    pub x1b8: f32,
    pub measurement_guy_4: f32,
    pub climb_related_x1c0: f32,
    pub _x1c4: Q4,
    pub scaled_diameter: [f32; 7],
    pub _x1e4: Q<92>,
    pub motion: MotionVectors,
    pub prev_motion: MotionVectors,
    pub _x3e0: Q<4>,
    pub speed_fac_df: f32,
    pub x3e8: f32,
    pub xe3c: f32,
    pub x3f0: f32,
    pub x3f4: f32,
    pub x3f8: f32,
    pub speed_fac_f: f32,
    pub x400: f32,
    pub x404: f32,
    pub speed_fac_d: f32,
    pub small_uphil_mass_factor: f32,
    pub big_uphill_mass_factor: f32,
    pub axis_selectee: Vec4,
    pub x424: f32,
    pub x428: f32,
    pub x42c: f32,
    pub x430: f32,
    pub x434: f32,
    pub angular_speed: f32,
    pub _x43c: Q<4>,
    pub roll_direction: Vec4,
    pub left_direction: Vec4,
    pub position: Vec4,
    pub prev_position: Vec4,
    pub _x480: Q<4>,
    pub x484: f32,
    pub _x488: Q<8>,
    pub x490: Vec4,
    pub _x4a0: Q<128>,
    pub mat_x520: Mat4,
    pub _x560: Q<64>,
    pub mat_x5a0: Mat4,
    pub mat_x5e0: Mat4,
    pub mat_x620: Mat4,
    pub x660: Vec4,
    pub x670: Vec4,
    pub _x680: Q<16>,
    pub x690: Vec4,
    pub _x6a0: Q<112>,
    pub transform: Mat4,
    pub divisors: Vec4,
    pub x760: f32,
    pub x764: Q4,
    pub climb_motion_scale: f32,
    pub climb_increment_guy: f32,
    pub x770: f32,
    pub _x774: Q<16>,
    pub another_climb_timer: i16,
    pub climb_timer: i16,
    pub time_spent_going_downhill: u16,
    pub time_spent_going_uphill: u16,
    pub ground_normal: Vec4,
    pub wall_normal: Vec4,
    pub x7ac: Vec4,
    pub x7bc: f32,
    pub x7c0: f32,
    pub x7c4: f32,
    pub _x7c8: Q<8>,
    pub x7d0: f32,
    pub _x7d4: Q<40>,
    pub radius_again: f32,
    pub x800: f32,
    pub x804: i16,
    pub x806: i16,
    pub _x808: Q<84>,
    pub impact_point: Vec4,
    pub sx: f32,
    pub sy: f32,
    pub sz: f32,
    pub _x878: Q<32>,
    pub climb_flag_thing: i16,
    pub water_ripple_timer: i16,
    pub water_droplet_timer: i16,
    pub splash_sfx_timer: i16,
    pub _x89a: Q<28>,
    pub x8bc: f32,
    pub _x8c0: Q<16>,
    pub indexed_guy: [IndexedGuy; 64],
    pub copied_from_indexed_guy: [IndexedGuy; 64],
    pub x38d0: u16,
    pub _x38d2: Q<2>,
    pub x38d4: Mat4,
    pub matrix_that_gets_reset: Mat4,
    pub local_pivot_pos: Vec4,
    pub vector_that_gets_reset_b: Vec4,
    pub pivot_position: Vec4,
    pub is_this_actually_used: Vec4,
    pub _x3994: Q<4>,
    pub guy_index: i32,
    pub guy_index_neighbor: i32,
    pub x39a0: f32,
    pub x39a4: f32,
    pub x39a8: f32,
    pub x39ac: f32,
    pub x39b0: f32,
    pub x39b4: f32,
    pub pivot_speed: f32,
    pub time_spent_standing_on_prop: i32,
    pub _x39c0: Q<24>,
    pub x39d8: Vec4,
    pub _x39e8: Q2,
    pub _x39ea: Q<6>,
    pub _x39f0: Q4,
    pub _x39f4: Q4,
    pub _x3948: Q<106>,
    pub _x3a62: Q2,
    pub _x3a64: Q<4>,
    pub _x3a68: Q8,
    pub x3a70: f32,
    pub x3a74: f32,
    pub x3a78: f32,
    pub x3a7c: f32,
    pub x3a80: f32,
    pub x3a84: Mat4,
    pub x3ac4: f32,
    pub x3ac8: f32,
    pub x3acc: i16,
    pub climb_timer_possibly: i16,
    pub timer_x3ad0: i16,
    pub _x3ad2: Q<4>,
    pub timer_x3ad6: i16,
    pub timer_x3ad8: i16,
    pub _x3ada: Q<66>,
    pub x3b1c: u8,
    pub _x3b1d: Q<28>,
    pub allow_set_pos_foo: bool,
    pub _x3b3a: Q<18>,
    pub possible_velocity: Vec4,
    pub _x3b5c: Q<8>,
    pub time_guy_2: f32,
    pub time_guy_1: f32,
    pub dash_state: u8,
    pub _x3b6d: Q1,
    pub dash_timer: i16,
    pub _x3b70: Q<20>,
    pub al_mode: u8,
    pub x3b85: u8,
    pub al_type: u8,
    pub _x3b87: Q1,
    pub angle_guy: f32,
    pub angle_add_guy: f32,
    pub angle_mul_guy: f32,
    pub sfx_0x2d_interval: i16,
    pub sfx_0x2d_timer: i16,
    pub _x3b98: Q<40>,
}

assert_size!(Katamari, 0x3bc0);

assert_offset!(Katamari, x88, 0x88);
assert_offset!(Katamari, x98, 0x98);
assert_offset!(Katamari, xa0, 0xa0);
assert_offset!(Katamari, airborne, 0xa4);
assert_offset!(Katamari, climbing, 0xa5);
assert_offset!(Katamari, climb_height_reached, 0xa6);
assert_offset!(Katamari, dwordflag_xa7, 0xa7);
assert_offset!(Katamari, hit_water, 0xab);
assert_offset!(Katamari, _xac, 0xac);
assert_offset!(Katamari, _xbc, 0xbc);
assert_offset!(Katamari, _xcb, 0xcb);
assert_offset!(Katamari, xf0, 0xf0);
assert_offset!(Katamari, x116, 0x116);
assert_offset!(Katamari, _x1c4, 0x1c4);
assert_offset!(Katamari, x3f8, 0x3f8);
assert_offset!(Katamari, x424, 0x424);
assert_offset!(Katamari, mat_x5a0, 0x5a0);
assert_offset!(Katamari, x670, 0x670);
assert_offset!(Katamari, time_spent_going_uphill, 0x78a);
assert_offset!(Katamari, x38d0, 0x38d0);
assert_offset!(Katamari, matrix_that_gets_reset, 0x3914);
assert_offset!(Katamari, _x3a64, 0x3a64);
assert_offset!(Katamari, _x3a68, 0x3a68);
assert_offset!(Katamari, _x3a68, 0x3a68);
assert_offset!(Katamari, timer_x3ad0, 0x3ad0);
assert_offset!(Katamari, _x3ad2, 0x3ad2);
assert_offset!(Katamari, _x3b70, 0x3b70);

pub struct PlayerSubStruct {}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct MotionVectors {
    pub a: Vec4,
    pub b: Vec4,
    pub c: Vec4,
    pub d: Vec4,
    pub e: Vec4,
    pub effective: Vec4,
    pub g: Vec4,
    pub h: Vec4,
    pub i: Vec4,
    /// Live representation of player joystick input.
    pub push_force: Vec4,
    pub gravity: Vec4,
    pub downhill: Vec4,
    /// Used for caluculating friction.
    /// X represents spin speed leftward, Z represents forward. (away from camera)
    pub local_spin: Vec4,
}

assert_size!(MotionVectors, 0xd0);

impl MotionVectors {
    pub fn as_slice_mut(&mut self) -> &mut [Vec4] {
        const N: usize = 13;
        let _size_assertion = std::mem::transmute::<Self, [Vec4; N]>;
        unsafe { std::mem::transmute::<_, &mut [Vec4; N]>(self) }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IndexedGuy {
    pub m00: Mat4,
    x40: Q<16>,
    pub three_floats: [f32; 3],
    pub x5c: bool,
    x5d: Q<3>,
}

assert_size!(IndexedGuy, 0x60);

#[repr(C)]
pub struct Prince {
    pub player_idx: u8,
    pub _x1: Q<3>,
    pub x4: u32,
    pub x8: f32,
    pub t: Vec4,
    pub x1c: Vec4,
    pub x2c: f32,
    pub _x30: Q<12>,
    pub x3c: f32,
    pub x40: f32,
    pub x44: f32,
    pub x48: f32,
    pub x4c: f32,
    pub x50: f32,
    pub x54: f32,
    pub x58: f32,
    pub x5c: Vec4,
    pub actual_yaw: f32,
    pub prince_flip_rate_guy: f32,
    pub x74: f32,
    pub x78: f32,
    pub x7c: f32,
    pub quotient_1: f32,
    pub quotient_2: f32,
    pub _x88: Q<8>,
    pub x90: f32,
    pub quotient_3: f32,
    pub quotient_4: f32,
    pub view_mode: u8, // ViewMode enum
    pub prevent_dashing: bool,
    pub _x9e: Q1,
    pub sticks_not_idle: bool,
    pub _xa0: Q<2>,
    pub ouji_state: OujiState,
    pub prev_ouji_state: OujiState,
    pub xd8: i16,
    pub xda: i16,
    pub xdc: i16,
    pub _xde: Q<2>,
    pub _xe0: Q4,
    pub xe4: f32,
    pub xe8: f32,
    pub xec: f32,
    pub xf0: f32,
    pub view_mode_flag_thing: i16,
    pub _xf6: Q<66>,
    pub transform: Mat4,
    pub x178: f32,
    pub _x17c: Q<52>,
    pub x1b0: Mat4, // position uncertain
    pub x1f0: f32,
    pub x1f4: f32,
    pub _x1f8: Q<84>,
    pub prince_flip_vector: Vec4,
    pub _x25c: Q<32>,
    pub _x27c: Q4,
    pub _x280: Q4,
    pub x284: f32,
    pub x288: f32,
    pub dividend_3: f32,
    pub x290: f32,
    pub divisor_3: i32,
    pub divisor_4: i32,
    pub dividend_1: f32,
    pub divisor_1: i32,
    pub x2a4: i32,
    pub x2a8: f32,
    pub dividend_2: f32,
    pub x2b0: f32,
    pub divisor_2: i32,
    pub x2b8: i32,
    pub x2bc: f32,
    pub x2c0: f32,
    pub x2c4: f32,
    pub x2c8: f32,
    pub x2cc: Q<4>,
    pub x2d0: f32,
    pub deadzone_for_turnaround: f32,
    pub x2d8: f32,
    pub dash_input_window: i32,
    pub x2e0: i32,
    pub stamina_limit: i32,
    pub max_exhaustion: i32,
    pub stamina_gain_amount: i32,
    pub stamina_gain_interval: i32,
    pub x2f4: i32,
    pub x2f8: f32,
    pub x2fc: i32,
    pub max_slope_stamina: f32,        // 100.0
    pub slope_stamina_drain_rate: f32, // 0.76?
    pub x308: Vec4,
    pub left_stick_x: f32,
    pub left_stick_z: f32,
    pub left_stick_y: f32,
    pub left_stick_w: f32,
    pub right_stick_x: f32,
    pub right_stick_z: f32,
    pub right_stick_y: f32,
    pub right_stick_w: f32,
    pub left_stick_direction: Vec4,
    pub right_stick_direction: Vec4,
    pub combined_stick_direction: Vec4,
    pub abs_left_stick_x: f32,
    pub abs_left_stick_z: f32,
    pub abs_left_stick_y: f32,
    pub abs_left_stick_w: f32,
    pub abs_right_stick_x: f32,
    pub abs_right_stick_z: f32,
    pub abs_right_stick_y: f32,
    pub abs_right_stick_w: f32,
    pub _x388: Q<16>,
    pub combined_stick_x: f32,
    pub combined_stick_z: f32,
    pub combined_stick_y: f32,
    pub combined_stick_w: f32,
    pub left_stick_magnitude: f32,
    pub right_stick_magnitude: f32,
    pub avg_stick_magnitude: f32,
    pub left_stick_angle: f32,
    pub right_stick_angle: f32,
    pub combined_stick_angle: f32,
    pub x3bc: f32,
    pub push_direction_z_ness: f32,
    pub x3c8: f32,
    pub x3cc: Mat4,
    pub x40c: Mat4,
    pub x44c: f32,
    pub x450: Q<32>,
    pub left_stick_masked_status: u8,
    pub right_stick_masked_status: u8,
    pub left_stick_status: u8,
    pub right_stick_status: u8,
    pub left_stick_mask: u8,
    pub right_stick_mask: u8,
    pub _x476: Q<2>,
    pub x478: i16,
    pub _x47a: Q<2>,
    pub dash_input_counter: i16,
    pub stamina: i16,
    pub exhaustion_timer: i16,
    pub is_exhausted: bool,
    pub _x483: Q1,
    pub stamina_gain_timer: i16,
    pub counter_x486: i16,
    pub slope_stamina: f32,
    pub axis_selector: u8,
    pub _x48d: Q<7>,
    pub x494: Vec4,
    pub x4a4: Vec4,
    pub angle: f32,
    pub _x4b8: Q<4>,
    pub counter_x4bc: i16,
    pub _x4be: Q<2>,
    pub x4c0: Mat4,
    pub _x500: Q4,
    pub x504: bool,
    pub _x505: Q<19>,
}

assert_size!(Prince, 0x518);
assert_offset!(Prince, x2c, 0x2c);
assert_offset!(Prince, prev_ouji_state, 0xbd);
assert_offset!(Prince, xd8, 0xd8);
assert_offset!(Prince, x1f0, 0x1f0);
assert_offset!(Prince, x2d0, 0x2d0);
assert_offset!(Prince, stamina_limit, 0x2e4);
assert_offset!(Prince, stamina_gain_interval, 0x2f0);
assert_offset!(Prince, x308, 0x308);
assert_offset!(Prince, _x388, 0x388);
assert_offset!(Prince, x40c, 0x40c);
assert_offset!(Prince, dash_input_counter, 0x47c);
assert_offset!(Prince, angle, 0x4b4);
assert_offset!(Prince, x4c0, 0x4c0);

#[repr(C)]
pub struct OujiState {
    pub dash_pending: bool,
    pub dash_base_requirement_met: bool,
    pub dash_spinning: bool,
    pub dash_stationary_spin: bool,
    pub flip_pending: u8,
    pub _x5: Q1,
    pub climbing: bool,
    pub _x7: Q1,
    pub animation_mode: u8,
    pub dash_active: bool,
    pub swimming: bool,
    pub xb: bool,
    pub xc: u8,
    pub xd: Q<9>,
    pub x16: u8,
    pub x17: bool,
    pub x18: Q1,
    pub x19: u8,
    pub x1a: Q1,
}

assert_size!(OujiState, 0x1b);
assert_offset!(OujiState, swimming, 0xa);
assert_offset!(OujiState, x1a, 0x1a);

#[repr(C)]
pub struct Camera {
    pub m_x0: Mat4,
    pub v_x40: Vec4,
    pub v_x50: Vec4,
    pub mission_param_ptr: *mut f32,
    pub _x68: Q4,
    pub x6c: f32,
    pub x70: f32,
    pub x74: Q<2>,
    pub timer_x76: i16,
    pub _x78: Q<5>,
    pub flag_x7d: bool,
    pub animation_mode: u8,
    pub flag_x7f: bool,
    pub prev_pos: Vec4,
    pub pos: Vec4,
    pub prev_look_at: Vec4,
    pub look_at: Vec4,
    pub index_into_sixty_vecs_c0: i16,
    pub index_into_sixty_vecs_c2: i16,
    pub sixty_positions: [Vec4; 60],
    pub sixty_directions: [Vec4; 60],
    pub hover_position: Vec4,
    pub v_x854: Vec4,
    pub v_x864: Vec4,
    pub v_x874: Vec4,
    pub top_view_transition_timer: u32,
    pub top_view_transition_duration: u32,
    pub x88c: f32,
    pub _x890: Q4,
    pub top_view_state: u8,
    pub _x895: Q<3>,
    pub x898: f32,
    pub x89c: Q4,
    pub _x8a0: Q<8>,
    pub v_x8a8: Vec4,
    pub x8b8: f32,
    pub x8bc: f32,
    pub x8c0: f32,
    pub _x8c4: Q<8>,
    pub x8cc: Q2,
    pub x8ce: u8,
    pub _x8cf: Q1,
    pub x8d0: Vec4,
    pub katamari_position: Vec4,
    pub x8f0: Vec4,
    pub x900: Vec4,
    pub distance_x910: f32,
    pub zoom_speed_x914: f32,
    pub some_kinda_timer: i16,
    pub _x91a: Q<2>,
    pub x91c: Vec4,
    pub _x92c: Q<2>,
    pub x92e: Q2,
    pub _x930: Q<2>,
    pub x932: Q2,
    pub _x934: Q<4>,
    pub x938: *mut u8,
    pub x940: Vec4,
    pub x950: Vec4,
    pub x960: Q2,
    pub _x962: Q<2>,
    pub x964: f32,
    pub _x968: Q<8>,
    pub some_kinda_func: unsafe extern "C" fn(),
    pub x978: Q4,
    pub _x97c: Q<4>,
}

assert_size!(Camera, 0x980);
assert_offset!(Camera, pos, 0x90);
assert_offset!(Camera, x89c, 0x89c);
assert_offset!(Camera, x8c0, 0x8c0);
assert_offset!(Camera, x8d0, 0x8d0);
assert_offset!(Camera, katamari_position, 0x8e0);
assert_offset!(Camera, x938, 0x938);
assert_offset!(Camera, some_kinda_func, 0x970);

#[repr(C)]
pub struct CameraTransform {
    pub x0: Mat4,
    pub x40: Mat4,
    pub x80: Vec4,
    pub x90: Vec4,
    pub xa0: Vec4,
    pub xb0: Vec4,
    pub xc0: Mat4,
    pub x100: Mat4,
    pub x140: Mat4,
    pub x180: f32, // scale?
    pub _x184: Q4,
}

assert_size!(CameraTransform, 0x188);

#[repr(C)]
pub struct Prop {
    pub mono_ctrl_idx: u16,
    pub mono_name_idx: u16,
    pub alternate_name: u16,
    pub has_parent: u8, // more flags
    pub attached: u8,
    pub flags_x8: u8,
    pub is_prop_stop: bool,
    pub is_disp_off: bool,
    pub xb: u8,
    pub xc: u8,
    pub xd: u8,
    pub xe: u8,
    pub timer_max: u8,
    pub dispatch_x10: u8,
    pub counter_x11: u8,
    pub _x12: Q<2>,
    pub alpha_ratio: f32,
    pub mono_move_type_num: u16,
    pub hit_on_area_num: u8,
    pub link_act_num: i8,
    pub _x1c: Q<1>,
    pub dispatch_x1d: u8,
    pub flags_x1e: u8,
    pub _x1f: u8,
    pub mono_shake_off_flag: bool,
    pub flag_x21: bool,
    pub _x22: Q<6>,
    pub multi_child: *mut Self,
    pub child: *mut Self,
    pub map_area: u8,
    pub _x39: Q<7>,
    pub pos_x40: Vec4,
    pub m_x450: Mat4,
    pub position: Vec4,
    pub rotation: Vec4,
    pub v_xb0: Vec4,
    pub pos_xc0: Vec4,
    pub rot_xd0: Vec4,
    pub vel_xe0: Vec4,
    pub rot_xf0: Vec4,
    pub rot_x100: Vec4,
    pub transform: Mat4,
    pub m_x150: Mat4,
    pub m_x190: Mat4,
    pub _x1d0: Q<0xa00>,
}

assert_size!(Prop, 0xbd0);
assert_offset!(Prop, has_parent, 0x6);
assert_offset!(Prop, flags_x8, 0x8);
assert_offset!(Prop, xb, 0xb);
assert_offset!(Prop, mono_shake_off_flag, 0x20);
assert_offset!(Prop, pos_x40, 0x40);
assert_offset!(Prop, vel_xe0, 0xe0);
assert_offset!(Prop, m_x190, 0x190);

#[repr(C)]
pub struct PropConstants {
    pub name: *const std::ffi::c_char,
    pub x8: f32,
    pub xc: f32,
    pub _x10: Q<7>,
    pub x18: i8,
    pub _x19: Q<5>,
    pub x1e: u8,
    pub _x1f: Q<1>,
    pub x20: u8,
    pub scream_sfx_type: u8,
    pub const_parent: u16,
    pub spawn_vfx_9: bool,
    pub prevent_pickup: bool,
    pub _x26: Q<2>,
    pub x28: u64,
}

assert_size!(PropConstants, 0x30);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Q<const N: usize>([u8; N]);

// aligned
pub struct Q1(pub u8);
pub struct Q2(pub u16);
pub struct Q4(pub u32);
pub struct Q8(pub u64);

// unaligned
pub struct Q2U(pub [u8; 2]);
pub struct Q4U(pub [u8; 4]);
pub struct Q8U(pub [u8; 8]);

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0, 0.0);
    pub const W: Self = Self::new(0.0, 0.0, 0.0, 1.0);
    pub const XYZ: Self = Self::new(1.0, 1.0, 1.0, 0.0);

    #[inline]
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    #[inline]
    pub const fn to_array(self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
    }

    #[inline]
    pub const fn to_simd(self) -> f32x4 {
        f32x4::from_array(self.to_array())
    }

    #[inline]
    pub fn sqrlen(&self) -> f32 {
        self.dot3(self)
    }

    #[inline]
    pub fn dot3(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    #[inline]
    pub fn len(&self) -> f32 {
        self.sqrlen().sqrt()
    }

    #[inline]
    pub fn xyz0(&self) -> Self {
        Self::new(self.x, self.y, self.z, 0.0)
    }

    #[inline]
    pub fn x0zw(&self) -> Self {
        Self::new(self.x, 0.0, self.y, self.z)
    }
}

impl Add for Vec4 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

impl AddAssign for Vec4 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}

impl Sub for Vec4 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        )
    }
}

impl Mul for Vec4 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x * rhs.x,
            self.y * rhs.y,
            self.z * rhs.z,
            self.w * rhs.w,
        )
    }
}

impl Mul<f32> for Vec4 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w)
    }
}

impl Div<f32> for Vec4 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs, self.w)
    }
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Mat4 {
    pub rows: [Vec4; 4],
}

impl Mat4 {
    pub const IDENTITY: Self = Self {
        rows: [
            Vec4::new(1.0, 0.0, 0.0, 0.0),
            Vec4::new(0.0, 1.0, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 1.0, 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        ],
    };
}
