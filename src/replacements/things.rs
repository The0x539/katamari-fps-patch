use super::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExtraThingState {
    // The vanilla game uses a u8, which isn't quite enough for our purposes
    pub rng_timer: u16,
    pub getup_timer: i16,
}

impl ExtraThingState {
    pub const fn new() -> Self {
        Self {
            rng_timer: 0,
            getup_timer: 0,
        }
    }

    pub unsafe fn of<'a>(thing: *mut Thing) -> &'a mut Self {
        unsafe { &mut EXTRA_THING_STATE[(*thing).mono_ctrl_idx as usize] }
    }
}

#[unsafe(no_mangle)]
static mut EXTRA_THING_STATE: [ExtraThingState; 4000] = [ExtraThingState::new(); 4000];

#[link(name = "native_replacements", kind = "static")]
unsafe extern "C" {
    pub fn melon_roll();
    pub fn animal_walk();
    pub fn animal_turn();
    pub fn hop_timer_reset();
    pub fn hop_timer_check();
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
    pub fn angle_move_towards();
    pub fn pursuit_angle_move_towards();
    pub fn animal_angle_move_towards();
    pub fn train_angle_move_towards();
    pub fn update_flee_timer_1();
    pub fn update_flee_timer_1b();
    pub fn update_flee_timer_2();
    pub fn other_hop_gravity();
    pub fn sine_bob();
    pub fn elevator_timer_reset();
    pub fn elevator_timer_update();
    pub fn elevator_height_update();
    pub fn teddy_bear_bowl_spin();
    pub fn basic_gravity();
    pub fn wobble_rate();
    pub fn flee_timer_update();
    pub fn update_collision_cooldown();
    pub fn getup_hop_gravity();
    pub fn flee_timer_update_state4();
    pub fn flee_velocity_1();
    pub fn flee_velocity_2();
    pub fn flee_velocity_3();
    pub fn flee_velocity_spin();
    pub fn spin_before_getup();
    pub fn jumboman_spin();
    pub fn windmill_spin();
    pub fn getup_flip();
    pub fn pendulum();
    pub fn wrecking_ball();
    pub fn bird_update_timer_rbx();
    pub fn bird_update_timer_rdx();
    pub fn bird_ascend_vertical_a();
    pub fn bird_ascend_vertical_b();
    pub fn bird_ascend_horizontal();
    pub fn bird_descend_horizontal();
    pub fn bird_descend_horizontal_simple();
    pub fn bird_descend_vertical_a();
    pub fn bird_descend_vertical_b();
    pub fn animal_update_partial_walk_timer();
    pub fn animal_update_full_walk_timer();
    pub fn scarecrow_sway();
    pub fn other_hop_timer_check();
    pub fn other_hop_timer_update();
    pub fn animal_flee_turn();
    pub fn kickball_position();
    pub fn kickball_rotation();
    pub fn kickball_deceleration();
    pub fn path_walker_position();
    pub fn fix_melon_jank();

    pub fn golfer_update_timer_a();
    pub fn golfer_start_timer_b();
    pub fn golfer_check_timer_b();
    pub fn golfer_update_timer_b();
    pub fn golfer_restart_both_timers();

    pub fn vortex_spin_a();
    pub fn vortex_spin_b();

    pub fn pinwheel_spin();

    pub fn balloon_desync_timer_update();

    pub fn swingset();

    pub fn bird_orbit();

    pub fn bamboo_fountain_timer_update();
    pub fn bamboo_fountain_motion();
}

pub unsafe extern "C" fn sink_rate() {
    unsafe {
        let k: *mut Katamari;
        let sink_rate: f32;
        asm!("", out("rcx") k, out("xmm0") sink_rate);
        (*k).sink_rate = sink_rate.powf(values::dt_ticks());
        asm!("", in("rcx") k);
    }
}

pub unsafe extern "C" fn start_rng_timer(t: *mut Thing) {
    unsafe {
        let rng: u32;
        asm!("", out("eax") rng);

        let min_time = 667; // 20 ticks -> ⅔ seconds
        let variance = 2100; // 0x3F ticks -> 2.1 seconds
        let timer = (rng % variance) + min_time;

        ExtraThingState::of(t).rng_timer = timer as u16;
        (*t).base_machine.rng_timer = 100; // just to make sure the branches work right
    }
}

pub unsafe extern "C" fn fish_timer_decrement() -> u8 {
    unsafe {
        let t: *mut Thing;
        asm!("", out("rdi") t);

        let state = ExtraThingState::of(t);
        state.rng_timer = state.rng_timer.saturating_sub(values::dt_millis() as u16);
        (*t).base_machine.rng_timer = if state.rng_timer > 0 { 100 } else { 0 };
        0
    }
}

pub unsafe extern "C" fn getup_timer_update() {
    unsafe {
        let t: *mut Thing;
        asm!("mov {}, rbx", out(reg) t);
        ExtraThingState::of(t).getup_timer += values::dt_millis();
        asm!("mov rbx, {}", in(reg) t);
    }
}

pub unsafe extern "C" fn getup_timer_check() {
    unsafe {
        let t: *mut Thing;
        asm!("mov {}, rbx", out(reg) t);
        let timer = ExtraThingState::of(t).getup_timer;
        asm! {
            "cmp {:x}, 267", // 8 ticks -> 267 milliseconds
            "mov rbx, {}",
            in(reg) timer,
            in(reg) t,
        }
    }
}

pub unsafe extern "C" fn flee_timer_start() {
    unsafe {
        let flee: *mut ();
        let mut timer: u16;
        asm! {
            "mov {}, rbx",
            out(reg) flee,
            out("ax") timer,
        };
        timer = (((timer as u64) * 1000) / 30) as u16;
        asm! {
            "mov rbx, {}",
            "mov [rbx + 0x30], ax",
            "mov byte ptr [rbx + 0x33], 0",
            in(reg) flee,
            in("ax") timer,
        }
    }
}

#[inline]
fn bird_timer(mono_ctrl_idx: u32) -> u16 {
    let mut timer = ps2::rng() as u32;
    timer += mono_ctrl_idx * 33; // should help achieve a roughly even distribution within the range
    timer %= 500; // corresponds to & 0xF (15 ticks)
    timer += 167;
    timer as u16
}

pub unsafe extern "C" fn bird_start_rng_timer_rax() {
    unsafe {
        let bird: *mut ();
        let mono_ctrl_idx: u32;
        asm! {
            "movzx {:e}, word ptr [rbx]",
            out(reg) mono_ctrl_idx,
            out("rax") bird,
        };
        asm! {
            "inc byte ptr [rax]",
            "mov word ptr [rax + 0x2], {:x}",
            in(reg) bird_timer(mono_ctrl_idx),
            in("rax") bird,
        }
    }
}

pub unsafe extern "C" fn bird_start_rng_timer_rdx() {
    unsafe {
        let bird: *mut ();
        let mono_ctrl_idx: u32;
        asm! {
            "movzx {:e}, word ptr [rbx]",
            out(reg) mono_ctrl_idx,
            out("rdx") bird,
        }
        asm! {
            "inc byte ptr [rdx]",
            "mov word ptr [rdx + 0x2], {:x}",
            in(reg) bird_timer(mono_ctrl_idx),
            in("rdx") bird,
        }
    }
}

unsafe fn animal_full_walk_timer(thing: *mut Thing) -> u16 {
    let mut timer = ps2::rng() as u32;
    timer += unsafe { (*thing).mono_ctrl_idx } as u32;
    timer *= 0x23a;
    timer %= 8567;
    timer += 4000;
    timer as u16
}

pub unsafe extern "C" fn animal_start_walk_timer() {
    unsafe {
        let thing: *mut Thing;
        let animal: *mut ();
        asm! {
            "mov {}, rbx",
            out(reg) thing,
            out("rdi") animal,
        }
        asm! {
            "mov [{} + 0x2], {:x}",
            in(reg) animal,
            in(reg) animal_full_walk_timer(thing),
            in("rdi") thing,
        }
    }
}

pub unsafe extern "C" fn animal_restart_walk_timer() {
    unsafe {
        let thing: *mut Thing;
        let animal: *mut ();
        asm! {
            "mov byte ptr [rbx + 0x1], 0",
            "mov {}, rbx",
            out(reg) animal,
            out("rcx") thing,
        }
        asm! {
            "mov [{} + 0x2], {:x}",
            in(reg) animal,
            in(reg) animal_full_walk_timer(thing),
        }
    }
}

fn animal_partial_walk_timer(range: u32, base: u32) -> u16 {
    let mut timer = ps2::rng() as u32;
    timer %= range;
    timer += base;
    timer as u16
}

pub unsafe extern "C" fn animal_reset_partial_walk_timer_30() {
    unsafe {
        let animal: *mut ();
        asm!("mov {}, rbx", out(reg) animal);
        asm! {
            "mov [{} + 0xa], {:x}",
            in(reg) animal,
            in(reg) animal_partial_walk_timer(1000, 1000),
        }
    }
}

pub unsafe extern "C" fn animal_reset_partial_walk_timer_60() {
    unsafe {
        let animal: *mut ();
        asm!("mov {}, rbx", out(reg) animal);
        asm! {
            "mov [{} + 0xa], {:x}",
            in(reg) animal,
            in(reg) animal_partial_walk_timer(2000, 4000),
        }
    }
}

pub unsafe extern "C" fn other_hop_timer_start() {
    unsafe {
        let mut timer: u32;
        let mut hop: *mut ();
        asm!("", out("rdx") hop, out("eax") timer);
        // Originally truncated to a byte, and 255 ticks is 8.5 seconds
        timer %= 8500;
        // Originally incremented by 0x14 = 20 ticks = ⅔ seconds
        // Not sure the original code actually worked quite as intended due to overflow.
        // It would've been a uniform distribution of the 256 possible values either way, right?
        // Oh well, guess I'm fixing a bug.
        timer += 667;
        asm! {
            "inc byte ptr [rdx + 0x9]", // trampoline-ish
            "mov word ptr [rdx + 0xa], {:x}",
            in(reg) timer,
            in("rdx") hop,
        }
    }
}

pub unsafe extern "C" fn balloon_desync_timer_start() {
    unsafe {
        asm!("", in("edx") ps2::rng() % 2000);
    }
}

#[unsafe(naked)]
pub unsafe extern "C" fn thing_bounce() {
    naked_asm! {
        "push rbx",
        "call {calc_vel}",
        "pop rbx",
        "movaps xmm0, [rbx + 0xe0]",
        "divps xmm0, [rip + {dt}]",
        "movaps [rbx + 0xe0], xmm0",
        "ret",
        calc_vel = sym ps2::thing_calc_velocity,
        dt = sym values::DT_TICKS,
    }
}
