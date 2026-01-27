use super::*;

unsafe extern "C" {
    pub fn orbit_a();
    pub fn orbit_b();
    pub fn zoom_out();
    pub fn size_threshold_animation_timer();
    pub fn size_threshold_animation_spin();
    pub fn size_threshold_animation_zoom();
    pub fn size_threshold_animation_other_zoom();
}

// TODO: For some reason, this might be too *slow* now, and the fade to white is still too fast.
pub unsafe extern "C" fn angel_zoom() {
    unsafe {
        asm!("push rcx");
        let distance = ps2::raw::angel_zoom_distance();
        let rate = *ps2::raw::angel_zoom_rate();
        *distance = rate.mul_add(values::dt_ticks(), *distance);
        asm!("pop rcx");
    }
}
