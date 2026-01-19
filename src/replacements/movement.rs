use super::*;

unsafe extern "C" {
    pub fn airborne_gravity();
    pub fn bumpy_ride();
    pub fn rolling_position();
    pub fn climbing_position();
    pub fn turn_radius();
    pub fn friction();
    pub fn push_force();
    pub fn climb_ascent_timer();
    pub fn climb_sustain_timer();
    pub fn climbing_ascent();
    pub fn bump_velocity();
}

#[inline]
fn pre_slope<'a>() -> (Vec4, &'a mut Katamari, &'a mut Prince) {
    unsafe {
        let (x, y, z);
        asm!("", out("xmm13") x, out("xmm14") y, out("xmm15") z);
        (
            Vec4::new(x, y, z, 0.0),
            ps2::current_katamari(),
            ps2::current_prince(),
        )
    }
}

pub unsafe extern "C" fn uphill() {
    let (downhill_v, k, p) = pre_slope();

    k.time_spent_going_downhill = 0;
    // This is probably one of the most likely things to overflow.
    // Fortunately, it's completely fine to saturate in this case, I think.
    k.time_spent_going_uphill = k
        .time_spent_going_uphill
        .saturating_add(values::dt_millis() as u16);

    k.slope_state = 1;

    // A larger katamari is easier to push uphill. (This number decreases with size.)
    let mass_factor = if p.push_direction_z_ness <= 0.0 {
        k.big_uphill_mass_factor
    } else {
        // The force is diminished even further if the player is pushing forward or backward.
        k.small_uphil_mass_factor
    };

    let gdot = ps2::gravity().dot3(&k.ground_normal);
    let theta = if (-1.0..=1.0).contains(&gdot) {
        gdot.acos()
    } else {
        0.0
    };

    let dt = values::dt_ticks();

    // As you push the katamari uphill, it gradually gets more difficult.
    // This meter depletes faster for a steeper slope,
    // and replenishes instantly upon reaching level terrain.
    let divisor = 0.74 * PI / 2.0;
    let drain_amount = (theta / divisor).clamp(0.0, 1.0);
    p.slope_stamina -= drain_amount * p.slope_stamina_drain_rate * dt;
    p.slope_stamina = p.slope_stamina.max(0.0);

    let time_factor = (k.time_spent_going_uphill as f32 / 20.0) * (30.0 / 1000.0);
    let time_factor = time_factor.min(1.0);

    let amount = time_factor * (k.diameter_cm / 50.0) * mass_factor * dt;
    k.motion.downhill += downhill_v * amount;
}

pub unsafe extern "C" fn downhill() {
    let (downhill_v, k, _) = pre_slope();

    k.time_spent_going_uphill = 0;
    k.time_spent_going_downhill = k
        .time_spent_going_downhill
        .saturating_add(values::dt_millis() as u16);

    k.slope_state = 2;

    let time_factor = (k.time_spent_going_downhill as f32 / 20.0) * (30.0 / 1000.0);
    let time_factor = time_factor.min(1.0);

    let amount = time_factor * (k.diameter_cm / 50.0) * values::dt_ticks();
    k.motion.downhill += downhill_v * amount;
}

// Turning in place with both sticks in opposite directions.
pub unsafe extern "C" fn fast_steer() {
    unsafe {
        asm! {
            "movss [rcx + 0x78], xmm1",
            "mulss xmm1, [rip + {dt}]",
            "mulss xmm1, {gkr}",
            "addss xmm1, [rcx + 0x6c]",
            dt = sym values::DT_TICKS,
            gkr = in(xmm_reg) ps2::g_katamari_rot(),
        }
    }
}

// Turning in place with one stick neutral and one stick forward or backward.
pub unsafe extern "C" fn slow_steer() {
    unsafe {
        let prince: *mut Prince;
        let direction: f32;
        asm! {
            "push rbx",
            "mov {}, rbx",
            out(reg) prince,
            out("xmm1") direction,
        };

        asm! {
            "movss [{prince} + 0x78], xmm1",
            "mulss xmm1, xmm0",
            "mulss xmm1, [rip + {dt}]",
            "addss xmm1, [rbx + 0x6c]",
            "pop rbx",

            in("xmm0") ps2::g_katamari_rot(),
            in("xmm1") direction,
            dt = sym values::DT_TICKS,
            prince = in(reg) prince,
        }
    }
}

// Steering while moving.
pub unsafe extern "C" fn gentle_steer() {
    unsafe {
        let prince: *mut Prince;
        asm!("mov {}, rbx", out(reg) prince);

        asm! {
            "mulss xmm1, [rip + {dt}]",
            "mulss xmm1, [{prince} + 0x78]",
            in("xmm1") ps2::g_katamari_rot(),
            dt = sym values::DT_TICKS,
            prince = in(reg) prince,
        }
    }
}

#[unsafe(naked)]
pub unsafe extern "C" fn spin_amount() {
    naked_asm! {
        "movss [rsp + 0x2c + 8], xmm9", // trampoline: finish storing the roll direction on the stack (what a waste)
        "mulss xmm2, [rip + {dt}]",     // important part: delta-time the "theta" arg
        "jmp {}",
        sym ps2::rotation_from_axis_angle,
        dt = sym values::DT_TICKS,
    }
}
