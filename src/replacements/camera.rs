use super::*;

unsafe extern "C" {
    pub fn orbit_a();
    pub fn orbit_b();
    pub fn zoom_out();
    pub fn size_threshold_animation_timer();
    pub fn size_threshold_animation_spin();
    pub fn size_threshold_animation_zoom();
    pub fn size_threshold_animation_other_zoom();
    pub fn angel_fade();
    pub fn camera_bump_timer_update();
    pub fn credits_zoom();
    pub fn credits_timer_update();
    pub fn credits_timer_zero();
    pub fn first_person_controls();
    pub fn update_shoot_timer();
    pub fn update_shoot_angle();
}

pub unsafe extern "C" fn angel_zoom() -> f32 {
    unsafe {
        asm!("push rcx");
        let distance = ps2::raw::angel_zoom_distance();
        let rate = ps2::raw::angel_zoom_rate();
        let dt = values::dt_ticks();
        *distance += *rate * dt;
        asm!("pop rcx");
        *distance // returned via xmm0, which is used again soon afterwards in the caller
    }
}

pub unsafe extern "C" fn credits_timer_start_a() {
    unsafe {
        *ps2::raw::credits_timer() = 30_000_i32;
    }
}

pub unsafe extern "C" fn credits_timer_start_b() {
    unsafe {
        *ps2::raw::credits_timer() = 214_800_i32;
    }
}
