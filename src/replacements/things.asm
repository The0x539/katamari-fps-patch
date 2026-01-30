section .text

extern PTR_THING_GRAVITY

global melon_spin
melon_spin:
	movss xmm0, [.theta]
	mulss xmm0, [DT_TICKS]
	addss xmm0, [rsi + 0x110]
	ret
	.theta: dd 0.15

global animal_walk
animal_walk:
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

global hop_timer_reset
hop_timer_reset:
	; Praying that the byte after this counter is unused,
	; at least by the NPCs we care about.
	mov word [rax + 0x90], 833 ; 25 ticks -> 833 ms
	ret

global hop_timer_update
hop_timer_update:
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

global hop_angle_update
hop_angle_update:
	movss xmm2, [rdi + 0x78]
	movss xmm1, [rdi + 0x74]
	vfmadd231ss xmm1, xmm0, [DT_TICKS]
	ret

global hop_position_update
hop_position_update:
	movups xmm3, [rcx + 0x90]
	movups xmm1, [DT_TICKS]
	vfmadd231ps xmm3, xmm1, [rcx + 0xe0]
	movups [rcx + 0x90], xmm3
	ret

global hop_gravity
hop_gravity:
	movss xmm3, [rdi + 0x88]           ; xmm3 = t->hop_grav_vel
	vfmadd132ss xmm0, xmm3, [DT_TICKS] ; xmm0 = (xmm0 * dt) + xmm3
	ret

; TODO: Maybe combine the preceding two functions into one, like this one?
global other_hop_gravity
other_hop_gravity:
	movss xmm1, [DT_TICKS]
	movss xmm0, [rdx + 0x14]
	vfmadd123ss xmm0, xmm1, [rdx + 0x10] ; xmm0 = (dt * accel) + velocity
	mov [rsp + 0xf8 + 8], rdi ; 'trampoline'?
	movss [rdx + 0x10], xmm0 ; store new velocity
	vfmadd123ss xmm0, xmm1, [rbx + 0x94] ; xmm0 = (dt * velocity) + position (bonus surprise switch to rbx)
	movss [rbx + 0x94], xmm0 ; store new position
	ret

global freefall_pos
freefall_pos:
	movss xmm5, [DT_TICKS]
	; xmm2 is the w component, so if xmm5 is no bueno, just tamper with this one
	vfmadd231ss xmm2, xmm5, [rcx + 0xec]
	vfmadd231ss xmm0, xmm5, [rcx + 0xe4]
	vfmadd231ss xmm1, xmm5, [rcx + 0xe8]

	; the stupid x component
	movss xmm3, [rcx + 0x90]
	vfmadd231ss xmm3, xmm5, [rcx + 0xe0]

	ret

global freefall_gravity
freefall_gravity:
	mov rdi, [PTR_THING_GRAVITY]
	movss xmm4, [rdi]
	vfmadd231ss xmm0, xmm4, [DT_TICKS]
	xor edi, edi
	ret

global freefall_spin_a
freefall_spin_a:
	movss xmm5, [DT_TICKS]
	vfmadd123ss xmm0, xmm5, [rcx + 0xa0 + 4*rax] ; xmm0 = (xmm0 * xmm5) + [current angle]
	ret

global freefall_spin_b
freefall_spin_b:
	movss xmm5, [DT_TICKS]
	vfmadd123ss xmm1, xmm5, [rcx + 0xa0 + 4*rax] ; xmm0 = (xmm0 * xmm5) + [current angle]
	ret

	
global freefall_spin_c
freefall_spin_c:
	movss xmm0, [rcx + 0xa4]
	movss xmm2, [.pi]
	vfmadd231ss xmm0, xmm1, [DT_TICKS]
	ret
	.pi: dd 3.14159265

global random_hop_motion
random_hop_motion:
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
global angle_move_towards
angle_move_towards:
	movss xmm1, [rdx]
	mulss xmm1, [DT_TICKS]
	xorps xmm0, xmm0
	comiss xmm1, xmm0
	movaps xmm2, xmm1
	addss xmm2, [rcx]
	ret

global pursuit_angle_move_towards
pursuit_angle_move_towards:
	movss xmm1, [rdx + 0x7c]
	mulss xmm1, [DT_TICKS]
	movss xmm0, [rdx + 0x78]
	movss xmm4, [.neg_tau]
	ret
	.neg_tau: dd -6.283185307

global animal_angle_move_towards
animal_angle_move_towards:
	movss xmm0, [rdi + 0x7c]
	mulss xmm0, [DT_TICKS]
	comiss xmm6, xmm0
	movss xmm2, [rdi + 0x78]
	ret

global train_angle_move_towards
train_angle_move_towards:
	mov r14, rcx
	movaps xmm2, xmm1
	mulss xmm2, [DT_TICKS]
	addss xmm2, [rdx + 0x10c]
	ret

global start_flee_timer
start_flee_timer:
	mov eax, 3000 ; 90 ticks -> 3 seconds
	mov [rdi + 0x124], ax
	ret

global update_flee_timer
update_flee_timer:
	sub ax, [DT_MILLIS]
	jns .not_negative
	xor eax, eax
	.not_negative:
	mov [rsp + 0x20 + 8], rbx ; trampoline
	ret

global sine_bob
sine_bob:
	movss xmm1, [DT_TICKS]
	vfmadd123ss xmm0, xmm1, [rbx + 0x10]
	ret

global elevator_timer_reset
elevator_timer_reset:
	; the trampoline instruction is written differently in each case in the original code,
	; but this does work in both cases
	inc byte [rbx + 0x2] ; state += 1
	imul eax, 1000
	xor edx, edx
	idiv dword [.thirty]
	mov [rbx + 0x2c], eax
	ret
	.thirty: dd 30

global elevator_timer_update
elevator_timer_update:
	sub eax, [DT_MILLIS]
	jns .not_negative
	xor eax, eax
	.not_negative:
	mov [rbx + 0x2c], eax
	ret

global elevator_height_update
elevator_height_update:
	movss xmm2, [DT_TICKS]
	vfmadd123ss xmm0, xmm2, [rbx + 0x20]
	ret

global teddy_bear_bowl_spin
teddy_bear_bowl_spin:
	movss xmm1, [.theta]
	vfmadd231ss xmm0, xmm1, [DT_TICKS]
	ret
	.theta: dd 0.05

global basic_gravity
basic_gravity:
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

global wobble_rate
wobble_rate:
	movss xmm2, [rsi + 0x4]
	mulss xmm2, [DT_TICKS]
	ret
