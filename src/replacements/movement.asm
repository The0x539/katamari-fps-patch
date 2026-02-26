section .text

fn airborne_gravity:
	; xmm5 is currently initialized to the katamari's f32@0x1a0 field,
	; which seems to be the gravitational acceleration value.
	movss xmm1, [DT_TICKS]
	vfmadd213ss xmm5, xmm1, [rbx + 0x2e4]
	ret

fn bumpy_ride:
	mulss xmm6, [DT_TICKS]
	movss [rdi + 0x39b8], xmm6
	ret

fn climbing_position:
        movaps xmm1, [rbx + 0x290] ; xmm1 = k->motion.f (effective velocity, EXCLUDING gravity)
	jmp increment_position

fn rolling_position:
	movaps xmm1, [rbx + 0x2b0] ; xmm1 = k->motion.h (effective velocity, including gravity)
	; fall through to increment_position

increment_position:
	; xmm6-8 contain the x/y/z of the "vel" variable,
	; but reading it back from memory is a lot easier than shuffling those into one XMM register.
	; Bonus: we can read the version that already includes gravity
	; xmm0-4 are clobbered by the original code.
	movaps xmm0, [rbx + 0x460]    ; xmm0 = k->position
	vfmadd231ps xmm0, xmm1, [DT_TICKS] ; xmm0 += xmm1 * dt
	movaps [rbx + 0x460], xmm0    ; k->position = xmm0
	ret

fn turn_radius:
	movss xmm2, [rsi + 0x78]
	mulss xmm2, [DT_TICKS]
	ret

fn friction:
	; xmm12 is the first register to be overwritten after the patched code
	movss xmm12, [DT_TICKS]
	vfmadd231ss xmm6, xmm12, [rbx + 0x300]
	vfmadd231ss xmm7, xmm12, [rbx + 0x304]
	vfmadd231ss xmm8, xmm12, [rbx + 0x308]
	ret

fn push_force:
	movss xmm1, [DT_TICKS]
	vfmadd213ss xmm6,  xmm1, [r12 + 0x0]
	vfmadd213ss xmm10, xmm1, [r12 + 0x4]
	vfmadd213ss xmm9,  xmm1, [r12 + 0x8]
	vfmadd213ss xmm15, xmm1, [r12 + 0xc]
	ret

fn climb_ascent_timer:
	mov [rsp + 0xc0 + 8], rsi
	mov si, [DT_MILLIS]
	add [rdx + 0x784], si
	xor esi, esi
	cmp word [rdx + 0x784], 1000 ; 1 second
	ret

fn climb_sustain_timer:
	movzx eax, word [rdx + 0x786]
	add ax, [DT_MILLIS]
	mov [rdx + 0x786], ax
	ret

fn climbing_ascent:
	; the original code is insanely complex to just subtract climb_amount from position.y
	; we start with climb_amount stored in xmm5
	mov rsi, [rsp + 0xC0 + 8]     ; annoying thing that was in the middle of the replaced code
	movaps xmm6, [rsp + 0xA0 + 8] ; likewise, and this one took way too long to find

	movss xmm3, [rbx + 0x464]           ; xmm3 = k->position.y
	vfnmadd231ss xmm3, xmm5, [DT_TICKS] ; xmm3 -= xmm5 * dt
	movss [rbx + 0x464], xmm3
	ret

fn bump_velocity:
	divss xmm0, [DT_TICKS]   ; important part
	lea r8, [rsp + 0x20 + 8] ; trampoline
	ret

fn spin_amount:
	movss [rsp + 0x2c + 8], xmm9 ; trampoline: finish storing the roll direction on the stack (what a waste)
	mulss xmm2, [DT_TICKS]       ; important part: delta-time the "theta" arg
	ret

fn steer_rbx:
	mulss xmm1, [DT_TICKS]
	addss xmm1, [rbx + 0x6c]
	ret

fn steer_rcx:
	mulss xmm1, [DT_TICKS]
	addss xmm1, [rcx + 0x6c]
	ret

fn prince_bump_timer_update:
	dec_dt ax
	xorps xmm0, xmm0
	ret

fn prince_forced_turn:
	movss xmm2, [rdi + 0x4b4]
	mulss xmm2, [DT_TICKS]
	ret

fn credits_sphere_walk:
	movaps xmm0, [rbx + 0x2b0]
	mulps xmm0, [DT_TICKS]
	ret

fn air_time_increment:
	add ax, [DT_MILLIS]
	jno .nof
	mov ax, 0x7fff
	.nof:
	mov [rbx + 0x114], ax
	ret

fn fall_time_increment:
	push ax
	mov ax, [rbx + 0x116]
	add ax, [DT_MILLIS]
	jno .nof
	mov ax, 0x7fff
	.nof:
	mov [rbx + 0x116], ax
	pop ax
	ret

fn fall_time_check:
	cmp word [rdx + 0x116], 333
	ret

fn bumped_by_thing:
	movaps xmm6, [rbx + 0xc0]
	subps xmm6, [rbx + 0x90]
	sqrlen xmm6
	sqrtss xmm6, xmm6
	; all the above is just simplifying the computation of the new speed, for fun
	divss xmm6, [DT_TICKS] ; the actual important bit
	ret

fn update_climb_cooldown:
	dec_dt ax
	mov [rcx + 0x118], ax
	ret
