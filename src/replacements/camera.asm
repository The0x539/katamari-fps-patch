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
	movaps xmm0, [rdx + 0x0]
	movaps xmm1, [rdx + 0x40]
	vfmadd231ps xmm0, xmm1, [DT_TICKS]
	movaps [rdx + 0x0], xmm0
	ret

fn size_threshold_animation_other_zoom:
	movaps xmm0, [rdx + 0x10]
	movaps xmm1, [rdx + 0x50]
	vfmadd231ps xmm0, xmm1, [DT_TICKS]
	movaps [rdx + 0x10], xmm0
	ret

; this isn't quite mathematically correct, but I am *way* too tired
; to figure out the analytical representation of what the original code doing.
; basically, it's similar to lerp smoothing,
; but there's a division by the number of ticks remaining in the mission timer,
; meaning the fade-to-white accelerates as the timer counts down towards zero.
;
; I'm hoping the difference between that behavior and my delta-timed version is small.
;
; ------------------------------------------------------------------------------------
;
; Returning to this, it turns out the math described above results in linear change.
; To observe this, plot the following in Desmos:
;
; f(0) = 0
; f(x) = f(x - 1) + (700 - f(x - 1))/(900 - x)
; ([0...900], f([0...900]))
;
; 900 represents the 30-second duration on the mission timer when the fade starts,
; such that `900 - x` is the remaining time.
;
; The second 0 in the first line represents the initial value of the thing being smoothed,
; while 700 represents the target value.
;
; Most importantly, observe that if you vary either of the zeroes in the first line,
; the start point moves around, but the plot remains linear.
;
;
;
; Hilarious.
;
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

fn first_person_controls:
	cvtdq2ps xmm3, xmm3 ; trampoline
	comiss xmm4, xmm1   ; trampoline
	mulss xmm1, [DT_TICKS]
	mulss xmm3, [DT_TICKS]
	ret

fn update_shoot_timer:
	mov dx, [rdi + 0x918]
	dec_dt dx
	mov [rdi + 0x918], dx
	ret

fn update_shoot_angle:
	mov ecx, 666
	sub ecx, eax
	cvtsi2ss xmm0, ecx
	divss xmm0, [.twothirds]
	ret
	.twothirds: dd 666.0
