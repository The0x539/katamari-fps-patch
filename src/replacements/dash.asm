section .text

global stamina_drain
stamina_drain:
	mov ax, [rdi + 0x47e]
	sub ax, [DT_MILLIS]
	mov [rdi + 0x47e], ax
	cmp ax, bp
	ret

global update_dash_input_timer
update_dash_input_timer:
	mov ax, [rdi + 0x478]  ; ax = prince->dash_input_timer
	sub ax, [DT_MILLIS]    ; ax -= DT_MILLIS
	jns .a                 ; if ax < 0 {
	xor eax, eax           ;     eax = 0
	.a:                    ; }
	mov [rdi + 0x478], ax  ; prince->dash_input_timer = ax
	cmp [MULTIPLAYER], sil ; annoying trampoline
	ret
