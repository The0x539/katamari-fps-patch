section .text

global orbit_a
orbit_a:
	movss xmm0, [.theta]
	movss xmm1, [rdi + 0x964]
	vfmadd231ss xmm1, xmm0, [DT_TICKS]
	ret
	.theta: dd 0.0418879

global orbit_b
orbit_b:
	movss xmm0, [.theta]
	movss xmm1, [rdi + r15 + 0xd3327c]
	vfmadd231ss xmm1, xmm0, [DT_TICKS]
	movss xmm0, [.neg_pi]
	ret
	.theta: dd -0.020943951
	.neg_pi: dd -3.14159265

global zoom_out
zoom_out:
	movss xmm6, [rdi + 0x910]          ; xmm6 = c->distance
	movss xmm0, [rdi + 0x914]          ; xmm0 = c->zoom_speed
	vfmadd231ss xmm6, xmm0, [DT_TICKS] ; xmm6 = xmm0 * dt + xmm6
	ret

global size_threshold_animation_timer
size_threshold_animation_timer:
	subss xmm0, [DT_TICKS]
	ret

global size_threshold_animation_spin
size_threshold_animation_spin:
	movss xmm3, [rdx + 0x978] ; trampoline
	vfmadd132ss xmm1, xmm3, [DT_TICKS]
	ret

global size_threshold_animation_zoom
size_threshold_animation_zoom:
	movups xmm0, [rdx + 0x0]
	movups xmm1, [rdx + 0x40]
	vfmadd231ps xmm0, xmm1, [DT_TICKS]
	movups [rdx + 0x0], xmm0
	ret

global size_threshold_animation_other_zoom
size_threshold_animation_other_zoom:
	movups xmm0, [rdx + 0x10]
	movups xmm1, [rdx + 0x50]
	vfmadd231ps xmm0, xmm1, [DT_TICKS]
	movups [rdx + 0x10], xmm0
	ret
