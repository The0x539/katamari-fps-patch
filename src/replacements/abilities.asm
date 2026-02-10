default rel

extern DT_MILLIS
extern DT_TICKS

section .text

fn flip_duration:
	addss xmm2, [rdx]
	mulss xmm2, [TICK_MS]
	cvttss2si eax, xmm2
	mov [rbx + 0x2d0], eax
	ret

fn flip_timer:
	mov edi, [DT_MILLIS]
	sub [rcx + 0x44c], edi
	xorps xmm11, xmm11
	movaps xmm0, [VEC_W]
	ret

fn katamari_view_ascend:
	mov eax, ebx
	add eax, [DT_MILLIS]
	movaps xmm6, xmm0 ; trampoline
	ret

fn katamari_view_descend:
	mov ebx, [rsi + 0x888] ; trampoline
	add edi, [DT_MILLIS]
	ret

fn spindash_gain_power:
	movss xmm2, [rbx + 0x420]
	movss xmm0, [rbx + 0x3a7c]
	vfmadd132ss xmm2, xmm0, [DT_TICKS]
	ret

fn spindash_spinning:
	mulps xmm0, [VEC_NEG]
	movups [rsp + 0x58 + 8], xmm0 ; trampoline
	mulss xmm2, [DT_TICKS]
	ret

section .rodata

VEC_W: dv 0.0, 0.0, 0.0, 1.0
VEC_NEG: dv -1.0, -1.0, -1.0, 1.0
TICK_MS: dd 33.33333333333333333
