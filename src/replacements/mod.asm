default rel

extern DT_MILLIS
extern DT_TICKS
extern MULTIPLAYER

%include 'abilities.asm'
%include 'cacophony.asm'
%include 'dash.asm'
%include 'camera.asm'
%include 'things.asm'
%include 'movement.asm'

section .text

; TODO: patch this directly into the original code to avoid the jump
; (possibly do similar for other position-independent code that's smaller than its replacement?)
global copy_matrix
copy_matrix:
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
        ret
