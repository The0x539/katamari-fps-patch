section .text

fn bang2:
	mov eax, [rbx + r8*8]
	add eax, [DT_MILLIS]
	mov [rbx + 8*r8], eax
	cmp eax, 333
	jg .a
	; and now for the tricky bit
	; I probably should have just rewritten a bigger chunk of the original function
	; maybe I'll do that on a later date
	add rsp, 0x8          ; return without returning
	mov rbx, [rsp + 0xf0] ; unwind the caller's stack frame
	add rsp, 0xe0
	pop rdi
	ret
	.a:
	; reset the timer
	sub eax, 333
	mov [rbx + 8*r8], eax
	ret

fn brake_dust_timer_update:
	mov cx, [r14 + 0x3b80]
	dec_dt cx
	mov [r14 + 0x3b80], cx
	ret
