.data
kMenu:
    .ascii "--- List Manager ---\n"
    .ascii "  a: append a new number\n"
    .ascii "  r: remove the last number\n"
    .ascii "  p: print the list\n"
    .asciiz "  q: quit\n"
kAppendPrompt: .asciiz "new number: "
kRemovedLast: .asciiz "removed last number\n"
kEmptyList: .asciiz "[]\n"
kAnyKeyToContinue: .asciiz "press any key to continue...\n"
kInvalidCommand: .asciiz "invalid command. try again\n"
kNothingToRemove: .asciiz "there's nothing to remove\n"
kAnsiClearScreen:
    .byte 0x1b
    .ascii "[2J"
    .byte 0x1b
    .asciiz "[H"

kMessageSleepTime: .word 750

kNumsPtr: .word 0
kNumsLength: .word 0

.text
main:
    main_prologue:
        addiu $sp, $sp, -0
    main_endprologue:

    main_loop0:
        addiu $v0, $0, 4
        la $a0, kMenu
        syscall
        addiu $v0, $0, 12
        syscall
        addu $s0, $0, $v0
        la $ra, main_endif0
        addiu $v0, $0, 4
        la $a0, kAnsiClearScreen
        syscall
        main_if0:
            beq $s0, 97, Append # 'a' == 97
        main_if0_elif0:
            beq $s0, 114, Remove # 'r' = 114
        main_if0_elif1:
            beq $s0, 112, Print # 'p' = 112
        main_if0_elif2:
            beq $s0, 113, main_endloop0 # 'q' = 113
        main_else0:
            addiu $v0, $0, 4
            la $a0, kInvalidCommand
            syscall
            addiu $v0, $0, 32
            lw $a0, kMessageSleepTime
            syscall
        main_endif0:
        addiu $v0, $0, 4
        la $a0, kAnsiClearScreen
        syscall
        j main_loop0
    main_endloop0:

    addiu $v0, $0, 9
    lw $t0, kNumsLength
    sll $t0, $t0, 2
    subu $a0, $0, $t0
    syscall

    main_epilogue:
        addiu $sp, $sp, 0
        addiu $v0, $0, 10
        syscall
    main_endepilogue:

Append:
    Append_prologue:
        addiu $sp, $sp, -0
    Append_endprologue:

    addiu $v0, $0, 4
    la $a0, kAppendPrompt
    syscall
    addiu $v0, $0, 5
    syscall
    addu $t0, $0, $v0
    addiu $v0, $0, 9
    addiu $a0, $0, 4
    syscall
    lw $t1, kNumsLength
    Append_if0:
        bgtz $t1, Append_endif0
        sw $v0, kNumsPtr
    Append_endif0:
    sw $t0, ($v0)
    addi $t1, $t1, 1
    sw $t1, kNumsLength

    Append_epilogue:
        addiu $sp, $sp, 0
        jr $ra
    Append_endepilogue:

Remove:
    Remove_prologue:
        addiu $sp, $sp, -0
    Remove_endprologue:

    lw $t0, kNumsLength
    Remove_if0:
        beqz $t0, Remove_else0
        addiu $t0, $t0, -1
        sw $t0, kNumsLength
        addiu $v0, $0, 9
        addiu $a0, $0, -4
        syscall
        la $a0, kRemovedLast
        j Remove_endif0
    Remove_else0:
        la $a0, kNothingToRemove
    Remove_endif0:
    addiu $v0, $0, 4
    syscall
    addiu $v0, $0, 32
    lw $a0, kMessageSleepTime
    syscall

    Remove_epilogue:
        addiu $sp, $sp, 0
        jr $ra
    Remove_endepilogue:

Print:
    Print_prologue:
        addiu $sp, $sp, -0
    Print_endprologue:

    Print_if0:
        lw $t1, kNumsLength
        bgtz $t1, Print_endif0
        addiu $v0, $0, 4
        la $a0, kEmptyList
        syscall
        j Print_postprint
    Print_endif0:

    addiu $v0, $0, 11
    addiu $a0, $0, 91 # '[' == 91
    syscall
    Print_for0:
        lw $t0, kNumsPtr
        sll $t1, $t1, 2
        addu $t1, $t1, $t0
    Print_for0_check:
        bge $t0, $t1, Print_endfor0
    Print_for0_body:
        addiu $v0, $0, 1
        lw $a0, ($t0)
        syscall
        addiu $v0, $0, 11
        addiu $a0, $0, 44 # ',' == 44
        syscall
    Print_for0_inc:
        addiu $t0, $t0, 4
        j Print_for0_check
    Print_endfor0:
    addiu $v0, $0, 11
    addiu $a0, $0, 93 # ']' == 93
    syscall
    addiu $a0, $0, 10 # '\n' == 10
    syscall

    Print_postprint:
        addiu $v0, $0, 4
        la $a0, kAnyKeyToContinue
        syscall
        addiu $v0, $0, 12
        syscall

    Print_epilogue:
        addiu $sp, $sp, 0
        jr $ra
    Print_endepilogue:
