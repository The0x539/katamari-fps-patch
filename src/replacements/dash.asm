section .text

global stamina_gain
stamina_gain:
	mov ax, [DT_MILLIS]
	add [rbx + 0x484], ax
	ret

global stamina_drain
stamina_drain:
	mov ax, [DT_MILLIS]
	sub [rdi + 0x47e], ax
	ret

global update_dash_input_timer
update_dash_input_timer:
	sub ax, [DT_MILLIS] ; ax -= DT_MILLIS
	jns .a              ; if ax < 0 {
	xor eax, eax        ;     eax = 0
	.a:                 ; }
	mov [rsp + 0x90 + 8], rbx ; trampoline
	ret

global prince_exhausted
prince_exhausted:
	mov cx, [DT_MILLIS]
	sub [rdx + 0x480], cx
	ret
