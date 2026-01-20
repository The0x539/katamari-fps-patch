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
	mov rax, [rsp + 8]
	mov word [rax + 0x90], 833 ; 25 ticks -> 833 ms
	mov word [rbx + 0x58e], 1
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
	movss xmm15, [rcx + 0x9c]
	movups xmm3, [rcx + 0x90]
	vbroadcastss xmm1, [DT_TICKS]
	vfmadd231ps xmm3, xmm1, [rcx + 0xe0]
	movups [rcx + 0x90], xmm3
	movss [rcx + 0x9c], xmm15
	ret

global hop_gravity
hop_gravity:
	movss xmm3, [rdi + 0x88]           ; xmm3 = t->hop_grav_vel
	vfmadd132ss xmm0, xmm3, [DT_TICKS] ; xmm0 = (xmm0 * dt) + xmm3
	mulss xmm2, [rdi + 0x28]
	movaps xmm3, xmm1
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
	movss xmm0, [rcx + 0xe4]
	mov rdi, [PTR_THING_GRAVITY]
	movss xmm4, [rdi]
	vfmadd231ss xmm0, xmm4, [DT_TICKS]
	xor edi, edi
	ret

; TODO: unify these two lmao, damn original compiler made two versions
; when I have more mental energy it should be possible
; to replace a slightly larger chunk such that these can use the same registers

global freefall_spin_a
freefall_spin_a:
	movzx eax, byte [rcx + 0x3d2] ; an "axis selector" or something
	movss xmm1, [rcx + 0x3d4]     ; the angular sped
	movaps xmm0, xmm1             ; two copies of the angular speed; one is used to calc decay
	movss xmm5, [DT_TICKS]
	vfmadd123ss xmm0, xmm5, [rcx + 0xa0 + 4*rax] ; xmm0 = (xmm0 * xmm5) + [current angle]
	ret

global freefall_spin_b
freefall_spin_b:
	movzx eax, byte [rcx + 0x3d2] ; an "axis selector" or something
	movss xmm0, [rcx + 0x3d4]     ; the angular sped
	movaps xmm1, xmm0             ; two copies of the angular speed; one is used to calc decay
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
