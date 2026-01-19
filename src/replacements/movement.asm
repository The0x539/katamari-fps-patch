section .text

global airborne_gravity
airborne_gravity:
	; xmm5 is currently initialized to the katamari's f32@0x1a0 field,
	; which seems to be the gravitational acceleration value.
	movss xmm1, [DT_TICKS]
	vfmadd213ss xmm5, xmm1, [rbx + 0x2e4]
	movss xmm1, [rbx + 0x2e8] ; trampoline to have enough room to fit the hook
	ret

global bumpy_ride
bumpy_ride:
	mulss xmm6, [DT_TICKS] ; the important bit
	movss [rdi + 0x39b8], xmm6
	; trampoline. extra 8 bytes for this function's return pointer
	lea rdx, [rsp + 0x28]
	ret

global rolling_position
rolling_position:
	movups xmm1, [rbx + 0x2b0] ; xmm1 = k->motion.h (effective velocity, including gravity)
	jmp increment_position

global climbing_position
climbing_position:
        movups xmm1, [rbx + 0x290] ; xmm1 = k->motion.f (effective velocity, EXCLUDING gravity)
        jmp increment_position

increment_position:
	; xmm6-8 contain the x/y/z of the "vel" variable,
	; but reading it back from memory is a lot easier than shuffling those into one XMM register.
	; Bonus: we can read the version that already includes gravity
	; xmm0-4 are clobbered by the original code.
	movups xmm0, [rbx + 0x460]    ; xmm0 = k->position
	movss xmm2, [rbx + 0x46c]     ; xmm2 = k->position.w

	vbroadcastss xmm3, [DT_TICKS] ; xmm3 = splat(dt)
	vfmadd231ps xmm0, xmm1, xmm3  ; xmm0 += xmm1 * xmm3

	movups [rbx + 0x460], xmm0    ; k->position = xmm0
	movss [rbx + 0x46c], xmm2     ; k->position.w = xmm2
	ret

global turn_radius
turn_radius:
	movss xmm2, [rsi + 0x78]
	mulss xmm2, [DT_TICKS]
	lea rdx, [rsp + 0x68]
	lea rcx, [rbp - 0x60]
	ret

global friction
friction:
	; xmm12 is the first register to be overwritten after the patched code
	movss xmm12, [DT_TICKS]
	vfmadd231ss xmm6, xmm12, [rbx + 0x300]
	vfmadd231ss xmm7, xmm12, [rbx + 0x304]
	vfmadd231ss xmm8, xmm12, [rbx + 0x308]
	ret

global push_force
push_force:
	movss xmm1, [DT_TICKS]
	vfmadd213ss xmm6,  xmm1, [r12 + 0x0]
	vfmadd213ss xmm10, xmm1, [r12 + 0x4]
	vfmadd213ss xmm9,  xmm1, [r12 + 0x8]
	vfmadd213ss xmm15, xmm1, [r12 + 0xc]
	ret

global climb_ascent_timer
climb_ascent_timer:
	mov [rsp + 0xc0 + 8], rsi
	mov si, [DT_MILLIS]
	add [rdx + 0x784], si
	xor esi, esi
	cmp word [rdx + 0x784], 1000 ; 1 second
	ret

global climb_sustain_timer
climb_sustain_timer:
	movzx eax, word [rdx + 0x786]
	add ax, [DT_MILLIS]
	mov [rdx + 0x786], ax
	ret

global climbing_ascent
climbing_ascent:
	; the original code is insanely complex to just subtract climb_amount from position.y
	; we start with climb_amount stored in xmm5
	mov rsi, [rsp + 0xC0 + 8]     ; annoying thing that was in the middle of the replaced code
	movaps xmm6, [rsp + 0xA0 + 8] ; likewise, and this one took way too long to find

	movss xmm3, [rbx + 0x464]           ; xmm3 = k->position.y
	vfnmadd231ss xmm3, xmm5, [DT_TICKS] ; xmm3 -= xmm5 * dt
	movss [rbx + 0x464], xmm3
	ret
