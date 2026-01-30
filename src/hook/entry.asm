default rel

extern TARGETS
extern OTHER_DLL_BASE

section .text

global trampoline_phase2
trampoline_phase2:
        push r8
        push r9
        push r10

        mov r8, [rsp + 0x18]
        sub r8, [OTHER_DLL_BASE]

        mov r9, r8
        shr r9, 0x10

        and r9, 0xFFFF
        and r8, 0xFFFF

        ; R9 now contains a low-entropy "small" index into the outer level of the TARGETS directory.
        ; R8 now contains a high-entropy "large" index into a page of the inner level.
        ; Index TARGETS by R9 to get the page pointer.
        lea r10, [TARGETS]
        mov r9, [r10 + r9*8]
        test r9, r9 ; Make sure the pointer we grabbed isn't null
        jz .oops

        ; R9 now contains a pointer to the appropriate "page" of hook targets.
        ; Index the page by R8 to get the function pointer.
        mov r8, [r9 + r8*8]
        test r8, r8
        jz .oops

        ; R8 now contains the appropriate function pointer.
        ; Restore the registers we used and jump to the target function.
        mov [current_target], r8
        pop r10
        pop r9
        pop r8
        jmp [current_target]

        .oops:
        pop r10
        pop r9
        pop r8
        ret

section .data
current_target: dq 0
