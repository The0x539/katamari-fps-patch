section .text

extern PTR_THING_GRAVITY
extern KICKBALL_DECEL

fn melon_roll:
	movss xmm0, [.theta]
	mulss xmm0, [DT_TICKS]
	addss xmm0, [rsi + 0x110]
	ret
	.theta: dd 0.15

fn animal_walk:
	; bit annoying that this went in the *middle* of the replaced code
	movups [rbp - 0x79], xmm3

	; I would love to make these properly SIMD,
	; but there's some really complicated swizzling right after this,
	; involving subtracting 5*size from the y component,
	; and then passing the vectors, using stack pointers, to some function.
	movss xmm3, [rbx + 0xe0]
	movss xmm0, [rbx + 0xe4]
	movss xmm1, [rbx + 0xe8]
	movss xmm4, [rbx + 0xec]

	movaps xmm2, xmm4 ; might need to swap these two registers

	mulss xmm3, [DT_TICKS]
	mulss xmm0, [DT_TICKS]
	mulss xmm1, [DT_TICKS]
	mulss xmm2, [DT_TICKS] ; probably not strictly necessary

	addss xmm3, [rbx + 0x90]
	addss xmm0, [rbx + 0x94]
	addss xmm1, [rbx + 0x98]
	addss xmm2, [rbx + 0x9c] ; what are we doing here?

	ret

fn hop_timer_reset:
	; Praying that the byte after this counter is unused,
	; at least by the NPCs we care about.
	mov word [rax + 0x90], 833 ; 25 ticks -> 833 ms
	ret

fn hop_timer_update:
	mov ax, [rcx + 0x90]
	cmp ax, 0
	jle .a

	sub ax, [DT_MILLIS]
	jns .b
	xor eax, eax
	.b:
	mov [rcx + 0x90], ax

	; the return of the cursed double return
	add rsp, 0x28
	pop rbx
	ret

	.a:
	ret

fn hop_angle_update:
	movss xmm2, [rdi + 0x78]
	movss xmm1, [rdi + 0x74]
	vfmadd231ss xmm1, xmm0, [DT_TICKS]
	ret

fn hop_position_update:
	movups xmm3, [rcx + 0x90]
	movups xmm1, [DT_TICKS]
	vfmadd231ps xmm3, xmm1, [rcx + 0xe0]
	movups [rcx + 0x90], xmm3
	ret

fn hop_gravity:
	movss xmm3, [rdi + 0x88]           ; xmm3 = t->hop_grav_vel
	vfmadd132ss xmm0, xmm3, [DT_TICKS] ; xmm0 = (xmm0 * dt) + xmm3
	ret

; TODO: Maybe combine the preceding two functions into one, like this one?
fn other_hop_gravity:
	movss xmm1, [DT_TICKS]
	movss xmm0, [rdx + 0x14]
	vfmadd123ss xmm0, xmm1, [rdx + 0x10] ; xmm0 = (dt * accel) + velocity
	mov [rsp + 0xf8 + 8], rdi ; 'trampoline'?
	movss [rdx + 0x10], xmm0 ; store new velocity
	vfmadd123ss xmm0, xmm1, [rbx + 0x94] ; xmm0 = (dt * velocity) + position (bonus surprise switch to rbx)
	movss [rbx + 0x94], xmm0 ; store new position
	ret

fn freefall_pos:
	movaps xmm1, [DT_TICKS]
	movaps xmm0, [rcx + 0x90]
	vfmadd231ps xmm0, xmm1, [rcx + 0xe0]
	movaps [rcx + 0x90], xmm0
	ret

fn freefall_gravity:
	mov rdi, [PTR_THING_GRAVITY]
	movss xmm4, [rdi]
	vfmadd231ss xmm0, xmm4, [DT_TICKS]
	xor edi, edi
	ret

fn freefall_spin_a:
	movss xmm5, [DT_TICKS]
	vfmadd123ss xmm0, xmm5, [rcx + 0xa0 + 4*rax] ; xmm0 = (xmm0 * xmm5) + [current angle]
	ret

fn freefall_spin_b:
	movss xmm5, [DT_TICKS]
	vfmadd123ss xmm1, xmm5, [rcx + 0xa0 + 4*rax] ; xmm0 = (xmm0 * xmm5) + [current angle]
	ret

	
fn freefall_spin_c:
	movss xmm0, [rcx + 0xa4]
	movss xmm2, [.pi]
	vfmadd231ss xmm0, xmm1, [DT_TICKS]
	ret
	.pi: dd 3.14159265

fn random_hop_motion:
	; We have pretty good freedom with with registers to use here,
	; as I don't think any subsequent code is dependent on XMM0/1/2.
	; As such, a simple mnemonic: XMM(n) is the nth derivative.
	movss xmm2, [rcx + 0x3e0]
	movss xmm1, [rcx + 0x3d8]
	movss xmm0, [rcx + 0x94]
	vfmadd231ss xmm1, xmm2, [DT_TICKS]
	vfmadd231ss xmm0, xmm1, [DT_TICKS]
	movss [rcx + 0x3d8], xmm1
	movss [rcx + 0x94], xmm0
	ret

; Please tell me there aren't any already-delta-timed uses of this.
; I'm going to guess not based on the usage of pointers for all the args.
fn angle_move_towards:
	movss xmm1, [rdx]
	mulss xmm1, [DT_TICKS]
	xorps xmm0, xmm0
	comiss xmm1, xmm0
	movaps xmm2, xmm1
	addss xmm2, [rcx]
	ret

fn pursuit_angle_move_towards:
	movss xmm1, [rdx + 0x7c]
	mulss xmm1, [DT_TICKS]
	movss xmm0, [rdx + 0x78]
	movss xmm4, [.neg_tau]
	ret
	.neg_tau: dd -6.283185307

fn animal_angle_move_towards:
	movss xmm0, [rdi + 0x7c]
	mulss xmm0, [DT_TICKS]
	comiss xmm6, xmm0
	movss xmm2, [rdi + 0x78]
	ret

fn train_angle_move_towards:
	mov r14, rcx
	movaps xmm2, xmm1
	mulss xmm2, [DT_TICKS]
	addss xmm2, [rdx + 0x10c]
	ret

fn update_flee_timer_1:
	dec_dt ax
	mov [rdi + 0x120], ax
	ret

fn start_flee_timer_2:
	mov eax, 3000 ; 90 ticks -> 3 seconds
	mov [rdi + 0x124], ax
	ret

fn update_flee_timer_2:
	dec_dt ax
	mov [rsp + 0x20 + 8], rbx ; trampoline
	ret

fn sine_bob:
	movss xmm1, [DT_TICKS]
	vfmadd123ss xmm0, xmm1, [rbx + 0x10]
	ret

fn elevator_timer_reset:
	; the trampoline instruction is written differently in each case in the original code,
	; but this does work in both cases
	inc byte [rbx + 0x2] ; state += 1
	imul eax, 1000
	xor edx, edx
	idiv dword [.thirty]
	mov [rbx + 0x2c], eax
	ret
	.thirty: dd 30

fn elevator_timer_update:
	dec_dt eax
	mov [rbx + 0x2c], eax
	ret

fn elevator_height_update:
	movss xmm2, [DT_TICKS]
	vfmadd123ss xmm0, xmm2, [rbx + 0x20]
	ret

fn teddy_bear_bowl_spin:
	movss xmm1, [.theta]
	vfmadd231ss xmm0, xmm1, [DT_TICKS]
	ret
	.theta: dd 0.05

fn basic_gravity:
	mov [rsp + 0x80 + 8], rsi ; trampoline

	; acceleration (xmm0 currently contains gravity)
	movups xmm1, [DT_TICKS] ; (we'll be using the whole vector soon)
	vfmadd123ss xmm0, xmm1, [rcx + 0xe4]
	movss [rcx + 0xe4], xmm0

	movaps xmm0, [rcx + 0x90]
	vfmadd231ps xmm0, xmm1, [rcx + 0xe0]
	movaps [rcx + 0x90], xmm0

	; the original function puts the final result back on the stack,
	; but I don't think it's actually used at any point lmao
	;movups [rsp + 0x20 + 8], xmm0

	ret

fn wobble_rate:
	movss xmm2, [rsi + 0x4]
	mulss xmm2, [DT_TICKS]
	ret

fn flee_timer_update:
	dec_dt ax
	mov [rdx + 0x30], ax
	ret

fn update_collision_cooldown:
	sub ax, [DT_MILLIS]
	mov [rbx], ax
	ret

fn getup_hop_gravity:
	movss xmm0, [.accel]
	movss xmm1, [DT_TICKS]

	; update velocity
	vfmadd123ss xmm0, xmm1, [rdx + 0x28] ; vel = (accel * dt) + vel
	movss [rdx + 0x28], xmm0

	; update position
	mulss xmm0, [.rate]
	vfmadd123ss xmm0, xmm1, [rcx + 0x94] ; pos = (vel * 0.8 * dt) + pos
	movss [rcx + 0x94], xmm0

	ret
	.accel: dd -0.72
	.rate: dd 0.8

fn flee_timer_update_state4:
	mov ax, [DT_MILLIS]
	neg ax
	ret

fn flee_velocity_1:
	mov rax, [PTR_THING_GRAVITY]
	movss xmm6, [rax]
	movaps xmm1, [DT_TICKS]

	; update gravitational velocity
	vfmadd123ss xmm6, xmm1, [rdi + 0x5c8]
	movss [rdi + 0x5c8], xmm6

	; update position using regular velocity
	movaps xmm0, [rdi + 0xe0]
	vfmadd123ps xmm0, xmm1, [rdi + 0x90]
	movaps [rdi + 0x90], xmm0

	; update position using gravitational velocity
	movss xmm0, xmm6
	vfmadd123ss xmm0, xmm1, [rdi + 0x94]
	movss [rdi + 0x94], xmm0

	; prepare "overall" y velocity for subsequent code???
	;addss xmm6, [rdi + 0xe4]

	ret

fn flee_velocity_2:
	movaps xmm1, [DT_TICKS]
	movaps xmm3, [rdi + 0xe0]
	vfmadd123ps xmm3, xmm1, [rdi + 0xc0]
	movaps [rdi + 0x90], xmm3
	ret

fn flee_velocity_3:
	movss xmm3, [rdi + 0x20]
	mulss xmm3, [DT_TICKS]
	ret

fn flee_velocity_spin:
	movss xmm3, [DT_TICKS]
	vfmadd231ss xmm6, xmm3, [rsi + 0x64]
	ret

fn spin_before_getup:
	movss xmm1, [rcx + 0xa4]
	movss xmm0, [DT_TICKS]
	vfmadd231ss xmm1, xmm0, [.theta]
	ret
	.theta: dd 0.15

fn getup_flip:
	movss xmm2, [DT_TICKS]
	comiss xmm1, xmm6
	jbe .increase

	vfnmadd231ss xmm1, xmm2, [.amount]
	maxss xmm0, xmm1
	ret

	.increase:
	vfmadd231ss xmm1, xmm2, [.amount]
	minss xmm0, xmm1
	ret

	.amount: dd 0.4

fn jumboman_spin:
	movss xmm1, [DT_TICKS]
	vfmadd123ss xmm0, xmm1, [rax + 0x20]
	ret

fn windmill_spin:
	movss xmm0, [DT_TICKS]
	vfmadd123ss xmm1, xmm0, [r8]
	ret

fn pendulum:
	movss xmm1, [DT_TICKS]
	vfmadd123ss xmm0, xmm1, [rbx + 0x40]
	ret

fn wrecking_ball:
	movss xmm2, [DT_TICKS]
	vfmadd123ss xmm0, xmm2, [rdi + 0x60]
	ret

fn bird_update_timer_rbx:
	dec_dt ax
	mov [rbx + 0x2], ax
	ret

fn update_flee_timer_1b:
fn bird_update_timer_rdx:
	dec_dt ax
	mov [rdx + 0x2], ax
	ret

fn bird_ascend_vertical_a:
fn bird_descend_vertical_b:
	movss xmm2, [DT_TICKS]
	vfmadd231ss xmm6, xmm2, [rbx + 0x64]
	ret

fn bird_ascend_horizontal:
	movss xmm2, [rbx + 0x6c]
	mulss xmm2, [DT_TICKS]
	ret

fn bird_ascend_vertical_b:
	mulss xmm5, [rbx + 0x68]
	mulss xmm5, [DT_TICKS]
	ret

fn bird_descend_horizontal:
	movups xmm0, [rbx + 0x10]
	vbroadcastss xmm1, [rbx + 0x78]
	mulps xmm1, [DT_TICKS]
	vfmadd123ps xmm0, xmm1, [rcx + 0x90]
	movups [rcx + 0x90], xmm0
	ret

; descend vertical "b" is the same code as ascend vertical "a"
fn bird_descend_vertical_a:
	movss xmm1, [.delta]
	vfnmadd231ss xmm0, xmm1, [DT_TICKS]
	ret
	.delta: dd 4.0

fn animal_update_partial_walk_timer:
	dec_dt ax
	mov [rbx + 0xa], ax
	ret

fn animal_update_full_walk_timer:
	dec_dt ax
	mov [rbx + 0x2], ax
	ret

fn scarecrow_sway:
	movss xmm0, [DT_TICKS]
	vfmadd123ss xmm1, xmm0, [rdi + 0x40]
	ret

fn other_hop_timer_check:
	mov ax, [rdx + 0xa]
	test ax, ax
	ret

; thank the heavens that there's a byte of padding for my usage
fn other_hop_timer_update:
	dec_dt ax
	mov [rdx + 0xa], ax
	ret

fn animal_flee_turn:
	movss xmm2, [DT_TICKS]
	vfmadd123ss xmm1, xmm2, [rdx + 0x74]
	ret

fn kickball_position:
	mov [rsp + 0x128 + 8], rbx
	movaps [rsp + 0x100 + 8], xmm6

	movups xmm0, [rax + 0xa0]
	movaps xmm1, [DT_TICKS]

	; gravity
	shufps xmm0, xmm0, 0b11_10_00_01   ; xyzw -> yxzw
	movss xmm6, xmm0                   ; xmm6 = y component of xmm0 (just got shuffled to low word)
	vfmadd231ss xmm6, xmm1, [.gravity] ; xmm6 += gravity * dt
	movss xmm0, xmm6                   ; move the updated y component back into xmm0's low word
	shufps xmm0, xmm0, 0b11_10_00_01   ; yxzw -> xyzw
	; now xmm6 contains vel.y + gravity, which gets used in a bit of code after the patch,
	; in order to actually "apply the gravitational acceleration"

	vfmadd123ps xmm0, xmm1, [rcx + 0x90]
	movups [rcx + 0x90], xmm0
	movups [rbp - 0x60], xmm0

	ret

	.gravity: dd -0.9

fn kickball_rotation:
	movss xmm0, [DT_TICKS]
	vfmadd123ss xmm2, xmm0, [rbx + 0xc0]
	ret

fn kickball_deceleration:
	movss xmm0, [rbx + 0xa0]
	mulss xmm0, [KICKBALL_DECEL]
	movss [rbx + 0xa0], xmm0

	movss xmm0, [rbx + 0xa8]
	mulss xmm0, [KICKBALL_DECEL]
	movss [rbx + 0xa8], xmm0
	ret

; I am extremely grateful that three different functions need this patch and use the same GPRs
fn path_walker_position_borked:
	movups xmm0, [rbx + 0xd0]

	mulps xmm0, [DT_TICKS]

	movups xmm1, [rbx + 0xc0]
	subps xmm1, [rdi + 0x90] ; xmm1 now contains the displacement towards the target position

	pcmpeqd xmm2, xmm2 ; xmm2 = [-1_i32; 4]
	pslld xmm2, 31     ; xmm2 = [1 << 31; 4]

	vandnps xmm3, xmm2, xmm0 ; xmm3 = abs(velocity)
	vandnps xmm4, xmm2, xmm1 ; xmm4 = abs(distance_from_target)

	cmpltps xmm4, xmm3        ; xmm4 = xmm4 < xmm3
	vblendvps xmm0, xmm0, xmm1, xmm4 ; xmm0 = IF xmm4 THEN xmm1 ELSE xmm0
	; End result: xmm0 contains either the velocity or the distance from the target,
	; whichever of the two values has a smaller *magnitude*, for each axis.

	addps xmm0, [rdi + 0x90] ; xmm0 now contains the new position
	movups [rdi + 0x90], xmm0
	ret

fn path_walker_position:
	movups xmm0, [rbx + 0xd0]
	movaps xmm1, [DT_TICKS]
	vfmadd123ps xmm0, xmm1, [rdi + 0x90]
	movups [rdi + 0x90], xmm0
	ret

fn fix_melon_jank:
	mulss xmm6, [DT_TICKS]
	ret

fn golfer_update_timer_a:
	dec_dt ax
	mov [rdx + 0x40], ax
	ret

fn golfer_start_timer_b:
	mov word [rdx + 0x42], 2250 ; 70 ticks -> 2.1 seconds (but this new magic number seems to actually animate properly for SOME reason)
	inc byte [rcx + 0x10] ; trampoline
	ret

fn golfer_check_timer_b:
	mov ax, [rdx + 0x42]
	test ax, ax
	ret

fn golfer_update_timer_b:
	dec_dt ax
	mov [rdx + 0x42], ax
	ret

fn golfer_restart_both_timers:
	mov word [rdx + 0x40], 3000 ; 90 ticks -> 3 seconds
	mov word [rdx + 0x42], 2250
	ret

fn vortex_spin_a:
	movss xmm2, [DT_TICKS]
	vfnmadd231ss xmm1, xmm2, [rdx + 0x44]
	ret

fn vortex_spin_b:
	; xmm2 is already populated from vortex_spin_a (the original function doesn't touch it)
	vfmadd123ss xmm0, xmm2, [rdx + 0x54]
	ret

fn pinwheel_spin:
	movss xmm1, [DT_TICKS]
	vfmadd123ss xmm0, xmm1, [rax + 0x70]
	ret

section .rodata

VEC_XYZ: dv 1.0, 1.0, 1.0, 0.0
VEC_NEG_ZERO: dv -0.0, -0.0, -0.0, -0.0
