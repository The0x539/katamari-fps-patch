section .text

fn stamina_gain:
	mov ax, [DT_MILLIS]
	add [rbx + 0x484], ax
	ret

fn stamina_drain:
	mov ax, [DT_MILLIS]
	sub [rdi + 0x47e], ax
	ret

fn update_dash_input_timer:
	sub ax, [DT_MILLIS] ; ax -= DT_MILLIS
	jns .a              ; if ax < 0 {
	xor eax, eax        ;     eax = 0
	.a:                 ; }
	mov [rsp + 0x90 + 8], rbx ; trampoline
	ret

fn prince_exhausted:
	mov cx, [DT_MILLIS]
	sub [rdx + 0x480], cx
	ret

fn multiplayer_dash_input_window:
	mov r9d, 200 ; 6 ticks -> 200 milliseconds
	ret
