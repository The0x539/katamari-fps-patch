default rel

extern DT_MILLIS
extern DT_TICKS
extern MULTIPLAYER

%macro dec_dt 1
	sub %1, [DT_MILLIS]
	jns .not_negative
	xor %1, %1
	.not_negative:
%endmacro

%macro fn 1
        global %1
        %1
%endmacro

%macro dv 4
        align 16
        %00:
                dd %1
                dd %2
                dd %3
                dd %4
%endmacro

%macro sqrlen 1
        dpps %1, %1, 0b0111_0001
%endmacro

%macro vsqrlen 2
        vdpps %1, %2, %2, 0b0111_0001
%endmacro

%include 'abilities.asm'
%include 'cacophony.asm'
%include 'dash.asm'
%include 'camera.asm'
%include 'things.asm'
%include 'movement.asm'

section .text

fn copy_matrix:
        sub rsp, 0x20
        movups [rsp], xmm0 ; Assumption: the original code doesn't use AVX
        movups [rsp + 0x10], xmm1

        vmovups ymm0, [rdx]
        vmovups ymm1, [rdx + 0x20]
        vmovups [rcx], ymm0
        vmovups [rcx + 0x20], ymm1

        mov rax, rcx

        vmovups xmm0, [rsp] ; VEX version, to zero the upper bits
        vmovups xmm1, [rsp + 0x10]
        add rsp, 0x20
        fn copy_matrix_end:
        ret
