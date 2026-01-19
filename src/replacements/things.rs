use super::*;

#[link(name = "native_replacements", kind = "static")]
unsafe extern "C" {
    pub fn melon_spin();
    pub fn animal_walk();
    pub fn hop_timer_reset();
    pub fn hop_timer_update();
    pub fn hop_angle_update();
    pub fn hop_position_update();
    pub fn hop_gravity();
}

pub unsafe extern "C" fn sink_rate() {
    unsafe {
        let k: *mut Katamari;
        asm!("", out("rdx") k);

        (*k).sink_rate = (*k).sink_rate.powf(values::DT_TICKS);

        // RCX and RAX would theoretically also be good to preserve,
        // but I think only RDX is strictly necessary based on the caller's register usage.
        asm!("", in("rdx") k);
    }
}
