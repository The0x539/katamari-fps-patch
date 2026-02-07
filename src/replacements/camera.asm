section .text

extern PTR_CREDITS_TIMER

fn orbit_a:
	movss xmm0, [.theta]
	movss xmm1, [rdi + 0x964]
	vfmadd231ss xmm1, xmm0, [DT_TICKS]
	ret
	.theta: dd 0.0418879

fn orbit_b:
	movss xmm0, [.theta]
	movss xmm1, [rdi + r15 + 0xd3327c]
	vfmadd231ss xmm1, xmm0, [DT_TICKS]
	movss xmm0, [.neg_pi]
	ret
	.theta: dd -0.020943951
	.neg_pi: dd -3.14159265

fn zoom_out:
	movss xmm6, [rdi + 0x910]          ; xmm6 = c->distance
	movss xmm0, [rdi + 0x914]          ; xmm0 = c->zoom_speed
	vfmadd231ss xmm6, xmm0, [DT_TICKS] ; xmm6 = xmm0 * dt + xmm6
	ret

fn size_threshold_animation_timer:
	subss xmm0, [DT_TICKS]
	ret

fn size_threshold_animation_spin:
	movss xmm3, [rdx + 0x978] ; trampoline
	vfmadd132ss xmm1, xmm3, [DT_TICKS]
	ret

fn size_threshold_animation_zoom:
	movups xmm0, [rdx + 0x0]
	movups xmm1, [rdx + 0x40]
	vfmadd231ps xmm0, xmm1, [DT_TICKS]
	movups [rdx + 0x0], xmm0
	ret

fn size_threshold_animation_other_zoom:
	movups xmm0, [rdx + 0x10]
	movups xmm1, [rdx + 0x50]
	vfmadd231ps xmm0, xmm1, [DT_TICKS]
	movups [rdx + 0x10], xmm0
	ret

; this isn't quite mathematically correct, but I am *way* too tired
; to figure out the analytical representation of what the original code doing.
; basically, it's similar to lerp smoothing,
; but there's a division by the number of ticks remaining in the mission timer,
; meaning the fade-to-white accelerates as the timer counts down towards zero.
;
; I'm hoping the difference between that behavior and my delta-timed version is small.
fn angel_fade:
	vfmadd231ss xmm2, xmm1, [DT_TICKS]
	comiss xmm2, xmm7 ; trampoline
	ret

fn camera_bump_timer_update:
	mov dx, [rdi + 0x8cc]
	dec_dt dx
	mov [rdi + 0x8cc], dx
	ret

fn credits_zoom:
	addss xmm1, [DT_TICKS]
	ret

fn credits_timer_update:
	mov rbx, [PTR_CREDITS_TIMER]
	movsx ecx, word [DT_MILLIS]
	sub [rbx], ecx
	ret

; Needed because the vanilla code only zeroes a word, not a dword.
fn credits_timer_zero:
	mov rbx, [PTR_CREDITS_TIMER]
	mov dword [rbx], 0
	xor rbx, rbx
	ret
