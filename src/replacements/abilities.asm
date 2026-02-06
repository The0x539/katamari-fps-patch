default rel

extern DT_MILLIS
extern DT_TICKS

section .text

global flip_duration
flip_duration:
	addss xmm2, [rdx]
	mulss xmm2, [TICK_MS]
	cvttss2si eax, xmm2
	mov [rbx + 0x2d0], eax
	ret

global flip_timer
flip_timer:
	mov edi, [DT_MILLIS]
	sub [rcx + 0x44c], edi
	xorps xmm11, xmm11
	movaps xmm0, [VEC4_W]
	ret

global katamari_view_ascend
katamari_view_ascend:
	mov eax, ebx
	add eax, [DT_MILLIS]
	movaps xmm6, xmm0 ; trampoline
	ret

global katamari_view_descend
katamari_view_descend:
	mov ebx, [rsi + 0x888] ; trampoline
	add edi, [DT_MILLIS]
	ret

global spindash_gain_power
spindash_gain_power:
	movss xmm2, [rbx + 0x420]
	movss xmm0, [rbx + 0x3a7c]
	vfmadd132ss xmm2, xmm0, [DT_TICKS]
	ret

global spindash_spinning
spindash_spinning:
	mulps xmm0, [VEC_NEG]
	movups [rsp + 0x58 + 8], xmm0 ; trampoline
	mulss xmm2, [DT_TICKS]
	ret

section .rodata

VEC4_W:
	dd 0.0
	dd 0.0
	dd 0.0
	dd 1.0

VEC_NEG:
	dd -1.0
	dd -1.0
	dd -1.0
	dd 1.0

TICK_MS:
	dd 33.33333333333333333
