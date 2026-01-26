use super::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExtraThingState {
    // The vanilla game uses a u8, which isn't quite enough for our purposes
    pub rng_timer: u16,
}

impl ExtraThingState {
    pub const fn new() -> Self {
        Self { rng_timer: 0 }
    }
}

#[unsafe(no_mangle)]
static mut EXTRA_THING_STATE: [ExtraThingState; 4000] = [ExtraThingState::new(); 4000];

#[link(name = "native_replacements", kind = "static")]
unsafe extern "C" {
    pub fn melon_spin();
    pub fn animal_walk();
    pub fn animal_turn();
    pub fn hop_timer_reset();
    pub fn hop_timer_update();
    pub fn hop_angle_update();
    pub fn hop_position_update();
    pub fn hop_gravity();
    pub fn freefall_gravity();
    pub fn freefall_pos();
    pub fn freefall_spin_a();
    pub fn freefall_spin_b();
    pub fn freefall_spin_c();
    pub fn random_hop_motion();
    // pub fn random_hop_timer();
    pub fn angle_move_towards();
    pub fn pursuit_angle_move_towards();
    pub fn animal_angle_move_towards();
    pub fn train_angle_move_towards();
    pub fn start_flee_timer();
    pub fn update_flee_timer();
}

pub unsafe extern "C" fn sink_rate() {
    unsafe {
        let k: *mut Katamari;
        asm!("", out("rdx") k);

        (*k).sink_rate = (*k).sink_rate.powf(values::dt_ticks());

        // RCX and RAX would theoretically also be good to preserve,
        // but I think only RDX is strictly necessary based on the caller's register usage.
        asm!("", in("rdx") k);
    }
}
