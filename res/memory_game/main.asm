.data
kFlippedOver:
	.byte 0x1b
	.asciiz "[90m"
kAnsiReset:
	.byte 0x1b
	.asciiz "[0m"
kAnsiClearScreen:
    .byte 0x1b
    .ascii "[2J"
    .byte 0x1b
    .asciiz "[H"
kTryToBeat0: .asciiz "Try to beat "
kTryToBeat1: .asciiz "'s time of "
kTableHeader: .asciiz "   0  1  2  3  4  5\n"
kGetGuessPrompt: .asciiz "Flip a card (format like 'row column'): "
kBadGuess: .asciiz "Invalid guess. Try again...\n"
kOpenFileError: .asciiz "Failed to open high score file\n"
kReadFileError: .asciiz "Failed to read high score\n"
kWriteFileError: .asciiz "Failed to write high score\n"
kYouWon: .asciiz "You won! :3\n"
kReportTime: .asciiz "Your final time was "
kSeconds: .asciiz " second(s)!\n"
kNewRecord: .asciiz "You just set a new record of "
kNamePrompt: .asciiz "What's your name? (max of 22 chars): "

kHighScorePath: .asciiz "high_score.dat"
kRngId: .word 69
kIntroSleepTime: .word 3000
kBadGuessSleepTime: .word 750
kNoMatchSleepTime: .word 1250
k1000: .double 1000

.align 3
kRecordHolder: .space 24
kNewRecordHolder: .space 24
kHighScore: .space 8

.text
main:
    main_prologue:
        addiu $sp, $sp, -28
    main_endprologue:

    addu $a0, $0, $sp
    jal SetTable
    jal GetHighScore
    addiu $v0, $0, 4
    la $a0, kAnsiClearScreen
    syscall
    la $a0, kTryToBeat0
    syscall
    la $a0, kRecordHolder
    syscall
    la $a0, kTryToBeat1
    syscall
    addiu $v0, $0, 3
    ldc1 $f12, kHighScore
    syscall
    addiu $v0, $0, 4
    la $a0, kSeconds
    syscall
    addiu $v0, $0, 32
    lw $a0, kIntroSleepTime
    syscall
    addiu $v0, $0, 30
    syscall
    sw $a0, 20($sp)
    sw $a1, 24($sp)
    addiu $s1, $0, 0 # open_guesses = 0
    main_for0:
        addiu $s0, $0, 0 # matches = 0
    main_for0_check:
        bge $s0, 9, main_endfor0
    main_for0_body:
        main_for0_body_if0:
            blt $s1, 2, main_for0_body_endif0
            lbu $s2, 18($sp)
            lbu $s3, 19($sp)
            addu $s2, $sp, $s2
            addu $s3, $sp, $s3
            lbu $s4, ($s2)
            lbu $s5, ($s3)
            main_for0_body_if0_if0:
                bne $s4, $s5, main_for0_body_if0_else0
                addiu $s0, $s0, 1
                j main_for0_body_if0_endif0
            main_for0_body_if0_else0:
                # TODO: maybe play a sound effect?
                addiu $v0, $0, 4
                la $a0, kAnsiClearScreen
                syscall
                addu $a0, $0, $sp
                jal DisplayTable
                addiu $v0, $0, 32
                lw $a0, kNoMatchSleepTime
                syscall
                andi $s4, $s4, 0xbf
                andi $s5, $s5, 0xbf
                sb $s4, ($s2)
                sb $s5, ($s3)
            main_for0_body_if0_endif0:
            # TODO: maybe play a sound effect?
            addiu $s1, $0, 0
            j main_for0_check
        main_for0_body_endif0:
        addiu $v0, $0, 4
        la $a0, kAnsiClearScreen
        syscall
        addu $a0, $0, $sp
        jal DisplayTable
        jal GetGuess
        main_for0_body_if1:
            bgez $v0, main_for0_body_endif1
            addiu $v0, $0, 32
            lw $a0, kBadGuessSleepTime
            syscall
            j main_for0_body_endif0
        main_for0_body_endif1:
        addu $a0, $0, $v0
        addu $a1, $0, $v1
        jal CoordinatesToIndex
        addu $t0, $sp, $v0
        lbu $t1, ($t0)
        ori $t2, $t1, 0x40
        main_for0_body_if2:
            bne $t1, $t2, main_for0_body_endif2
            addiu $v0, $0, 4
            la $a0, kBadGuess
            syscall
            addiu $v0, $0, 32
            lw $a0, kBadGuessSleepTime
            syscall
            j main_for0_body_endif0
        main_for0_body_endif2:
        sb $t2, ($t0)
        addiu $t0, $sp, 18
        addu $t0, $t0, $s1
        sb $v0, ($t0)
        addiu $s1, $s1, 1
        j main_for0_check
    main_endfor0:
    addiu $v0, $0, 30
    syscall
    lw $a2, 20($sp)
    lw $a3, 24($sp)
    subu $v1, $a1, $a3
    sltu $t1, $a1, $v1
    subu $v0, $a0, $a2
    subu $v0, $v0, $t1
    mtc1 $v0, $f0
    cvt.d.w $f20, $f0
    ldc1 $f4, k1000
    div.d $f20, $f20, $f4

    addiu $v0, $0, 4
    la $a0, kAnsiClearScreen
    syscall
    addu $a0, $0, $sp
    jal DisplayTable
    # TODO: maybe play a sound effect?
    addiu $v0, $0, 4
    la $a0, kYouWon
    syscall
    main_if0:
        ldc1 $f4, kHighScore
        c.lt.d $f20, $f4
        bc1f main_else0
        sdc1 $f20, kHighScore
        la $a0, kNewRecord
        syscall
        addiu $v0, $0, 3
        mov.d $f12, $f20
        syscall
        addiu $v0, $0, 4
        la $a0, kSeconds
        syscall
        la $a0, kNewRecordHolder
        jal GetName
        jal SetHighScore
        j main_endif0
    main_else0:
        la $a0, kReportTime
        syscall
        addiu $v0, $0, 3
        mov.d $f12, $f20
        syscall
        addiu $v0, $0, 4
        la $a0, kSeconds
        syscall
    main_endif0:

    main_epilogue:
        addiu $sp, $sp, 28
        addiu $v0, $0, 10
        syscall
    main_endepliogue:

CoordinatesToIndex:
    CoordinatesToIndex_prologue:
    CoordinatesToIndex_endprologue:

    addiu $at, $0, 6
    mul $v0, $a0, $at
    addu $v0, $v0, $a1

    CoordinatesToIndex_epilogue:
        jr $ra
    CoordinatesToIndex_endepilogue:

DisplayTable:
    DisplayTable_prologue:
        addiu $sp, $sp, -8
        sw $ra, ($sp)
        sw $s0, 4($sp)
        addu $s0, $0, $a0
    DisplayTable_endprologue:

    addiu $v0, $0, 4
    la $a0, kTableHeader
    syscall
    addu $a0, $0, $s0
    addiu $a1, $0, 0
    jal DisplayTableRow
    addiu $v0, $0, 11
    addiu $a0, $0, 10
    syscall
    addiu $a0, $s0, 6
    addiu $a1, $0, 1
    jal DisplayTableRow
    addiu $v0, $0, 11
    addiu $a0, $0, 10
    syscall
    addiu $a0, $s0, 12
    addiu $a1, $0, 2
    jal DisplayTableRow
    addiu $v0, $0, 11
    addiu $a0, $0, 10
    syscall

    DisplayTable_epilogue:
        lw $s0, 4($sp)
        lw $ra, ($sp)
        addiu $sp, $sp, 8
        jr $ra
    DisplayTable_endepilogue:

DisplayTableRow:
    DisplayTableRow_prologue:
        addiu $sp, $sp, -20
        sw $ra, ($sp)
        sw $s0, 4($sp)
        sw $s1, 8($sp)
        sw $s2, 12($sp)
        addu $s0, $0, $a0
        addu $s1, $0, $a1
    DisplayTableRow_endprologue:

    addiu $v0, $0, 11
    addiu $a0, $0, 32
    syscall
    syscall
    DisplayTableRow_for0:
        addiu $s2, $0, 0
    DisplayTableRow_for0_check:
        bge $s2, 6, DisplayTableRow_endfor0
    DisplayTableRow_for0_body:
        addu $t0, $s0, $s2
        lbu $a0, ($t0)
        sw $s2, 16($sp)
        jal DisplayCardTop
        lw $s2, 16($sp)
    DisplayTableRow_for0_inc:
        addiu $s2, $s2, 1
        j DisplayTableRow_for0_check
    DisplayTableRow_endfor0:
    addiu $v0, $0, 11
    addiu $a0, $0, 10
    syscall
    addiu $v0, $0, 1
    addu $a0, $0, $s1
    syscall
    addiu $v0, $0, 11
    addiu $a0, $0, 32
    syscall
    DisplayTableRow_for1:
        addiu $s2, $0, 0
    DisplayTableRow_for1_check:
        bge $s2, 6, DisplayTableRow_endfor1
    DisplayTableRow_for1_body:
        addu $t0, $s0, $s2
        lbu $a0, ($t0)
        sw $s2, 16($sp)
        jal DisplayCardMiddle
        lw $s2, 16($sp)
    DisplayTableRow_for1_inc:
        addiu $s2, $s2, 1
        j DisplayTableRow_for1_check
    DisplayTableRow_endfor1:
    addiu $v0, $0, 11
    addiu $a0, $0, 10
    syscall
    addiu $a0, $0, 32
    syscall
    syscall
    DisplayTableRow_for2:
        addiu $s2, $0, 0
    DisplayTableRow_for2_check:
        bge $s2, 6, DisplayTableRow_endfor2
    DisplayTableRow_for2_body:
        addu $t0, $s0, $s2
        lbu $a0, ($t0)
        sw $s2, 16($sp)
        jal DisplayCardBottom
        lw $s2, 16($sp)
    DisplayTableRow_for2_inc:
        addiu $s2, $s2, 1
        j DisplayTableRow_for2_check
    DisplayTableRow_endfor2:

    DisplayTableRow_epilogue:
        lw $s2, 12($sp)
        lw $s1, 8($sp)
        lw $s0, 4($sp)
        lw $ra, ($sp)
        addiu $sp, $sp, 20
        jr $ra
    DisplayTableRow_endepilogue:

DisplayCardTop:
    DisplayCardTop_prologue:
    	addiu $sp, $sp, -8
    DisplayCardTop_endprologue:

    andi $t0, $a0, 0x80 # not_blank
    beqz $t0, DisplayCardTop_epilogue
    andi $t1, $a0, 0x40 # visible
    andi $t2, $a0, 0x0c # suit
    srl $t2, $t2, 2
    andi $t3, $a0, 0x03 # color
    DisplayCardTop_if0:
        beqz $t1, DisplayCardTop_else0
        addiu $t9, $0, 0x1b
        sb $t9, 0($sp)
        addiu $t9, $0, 0x5b # 0x5b = '['
        sb $t9, 1($sp)
        addiu $t9, $0, 0x33 # 0x33 = '3'
        sb $t9, 2($sp)
        addiu $t9, $t3, 0x31 # 0x31 = '1'
        sb $t9, 3($sp)
        addiu $t9, $0, 0x6d # 0x6d = 'm'
        sb $t9, 4($sp)
        addiu $v0, $0, 4
        addu $a0, $0, $sp
        syscall
        j DisplayCardTop_endif0
    DisplayCardTop_else0:
        addiu $v0, $0, 4
        la $a0, kFlippedOver
        syscall
    DisplayCardTop_endif0:
    addiu $v0, $0, 11
    addiu $a0, $0, 0x250c # 0x250c = '┌'
    syscall
    addiu $a0, $0, 0x2500 # 0x2500 = '─'
    syscall
    addiu $a0, $0, 0x2510 # 0x2510 = '┐'
    syscall
    addiu $v0, $0, 4
    la $a0, kAnsiReset
    syscall

    DisplayCardTop_epilogue:
    	addiu $sp, $sp, 8
        jr $ra
    DisplayCardTop_endepilogue:

DisplayCardMiddle:
    DisplayCardMiddle_prologue:
        addiu $sp, $sp, -8
    DisplayCardMiddle_endprologue:

    andi $t0, $a0, 0x80 # not_blank
    beqz $t0, DisplayCardMiddle_epilogue
    andi $t1, $a0, 0x40 # visible
    andi $t2, $a0, 0x0c # suit
    srl $t2, $t2, 2
    andi $t3, $a0, 0x03 # color
    DisplayCardMiddle_if0:
        beqz $t1, DisplayCardMiddle_else0
        addiu $t8, $t2, 0x41 # 0x41 = 'A'
        addiu $t9, $0, 0x1b
        sb $t9, 0($sp)
        addiu $t9, $0, 0x5b # 0x5b = '['
        sb $t9, 1($sp)
        addiu $t9, $0, 0x33 # 0x33 = '3'
        sb $t9, 2($sp)
        addiu $t9, $t3, 0x31 # 0x31 = '1'
        sb $t9, 3($sp)
        addiu $t9, $0, 0x6d # 0x6d = 'm'
        sb $t9, 4($sp)
        addiu $v0, $0, 4
        addu $a0, $0, $sp
        syscall
        j DisplayCardMiddle_endif0
    DisplayCardMiddle_else0:
        addiu $t8, $0, 0x23 # 0x23 = '#'
        addiu $v0, $0, 4
        la $a0, kFlippedOver
        syscall
    DisplayCardMiddle_endif0:
    addiu $v0, $0, 11
    addiu $a0, $0, 0x2502 # 0x2502 = '│'
    syscall
    addu $a0, $0, $t8
    syscall
    addiu $a0, $0, 0x2502 # 0x2502 = '│'
    syscall
    addiu $v0, $0, 4
    la $a0, kAnsiReset
    syscall

    DisplayCardMiddle_epilogue:
        addiu $sp, $sp, 8
        jr $ra
    DisplayCardMiddle_endepilogue:

DisplayCardBottom:
    DisplayCardBottom_prologue:
    	addiu $sp, $sp, -8
    DisplayCardBottom_endprologue:

    andi $t0, $a0, 0x80 # not_blank
    beqz $t0, DisplayCardBottom_epilogue
    andi $t1, $a0, 0x40 # visible
    andi $t2, $a0, 0x0c # suit
    srl $t2, $t2, 2
    andi $t3, $a0, 0x03 # color
    DisplayCardBottom_if0:
        beqz $t1, DisplayCardBottom_else0
        addiu $t9, $0, 0x1b
        sb $t9, 0($sp)
        addiu $t9, $0, 0x5b # 0x5b = '['
        sb $t9, 1($sp)
        addiu $t9, $0, 0x33 # 0x33 = '3'
        sb $t9, 2($sp)
        addiu $t9, $t3, 0x31 # 0x31 = '1'
        sb $t9, 3($sp)
        addiu $t9, $0, 0x6d # 0x6d = 'm'
        sb $t9, 4($sp)
        addiu $v0, $0, 4
        addu $a0, $0, $sp
        syscall
        j DisplayCardBottom_endif0
    DisplayCardBottom_else0:
        addiu $v0, $0, 4
        la $a0, kFlippedOver
        syscall
    DisplayCardBottom_endif0:
    addiu $v0, $0, 11
    addiu $a0, $0, 0x2514 # 0x2514 = '└'
    syscall
    addiu $a0, $0, 0x2500 # 0x2500 = '─'
    syscall
    addiu $a0, $0, 0x2518 # 0x2518 = '┘'
    syscall
    addiu $v0, $0, 4
    la $a0, kAnsiReset
    syscall

    DisplayCardBottom_epilogue:
    	addiu $sp, $sp, 8
        jr $ra
    DisplayCardBottom_endepilogue:

GetGuess:
    GetGuess_prologue:
        addiu $sp, $sp, -28
        sw $ra, ($sp)
        sw $s0, 4($sp)
        sw $s1, 8($sp)
    GetGuess_endprologue:

    addiu $v0, $0, 4
    la $a0, kGetGuessPrompt
    syscall
    addiu $v0, $0, 8
    addiu $a0, $sp, 12
    addiu $a1, $0, 16
    syscall
    lbu $t0, 12($sp)
    addiu $s0, $t0, -48
    bltz $s0, GetGuess_badguess
    bge $s0, 3, GetGuess_badguess
    lbu $t0, 13($sp)
    bne $t0, 32, GetGuess_badguess
    lbu $t0, 14($sp)
    addiu $s1, $t0, -48
    bltz $s1, GetGuess_badguess
    bge $s1, 6, GetGuess_badguess
    lbu $t0, 15($sp)
    bne $t0, 10, GetGuess_badguess
    addu $v0, $0, $s0
    addu $v1, $0, $s1
    j GetGuess_epilogue

    GetGuess_badguess:
        addiu $v0, $0, 4
        la $a0, kBadGuess
        syscall
        addiu $v0, $0, -1
        addiu $v1, $0, -1

    GetGuess_epilogue:
        lw $s1, 8($sp)
        lw $s0, 4($sp)
        lw $ra, ($sp)
        addiu $sp, $sp, 28
        jr $ra
    GetGuess_endepilogue:

GetHighScore:
    GetHighScore_prologue:
        addiu $sp, $sp, -4
        sw $s0, ($sp)
    GetHighScore_endprologue:

    addiu $v0, $0, 13
    la $a0, kHighScorePath
    addiu $a1, $0, 0 # read-only
    syscall
    bltz $v0, ThrowOpenFileError
    addu $s0, $0, $v0
    addiu $v0, $0, 14
    addu $a0, $0, $s0
    la $a1, kRecordHolder
    addiu $a2, $0, 24
    syscall
    bne $v0, $a2, ThrowReadFileError
    addiu $v0, $0, 14
    la $a1, kHighScore
    addiu $a2, $0, 8
    syscall
    bne $v0, $a2, ThrowReadFileError
    addiu $v0, $0, 16
    syscall

    GetHighScore_epilogue:
        lw $s0, ($sp)
        addiu $sp, $sp, 4
        jr $ra
    GetHighScore_endepilogue:

GetName:
    GetName_prologue:
        addiu $sp, $sp, -4
        sw $s0, ($sp)
        addu $s0, $0, $a0
    GetName_endprologue:

    addiu $v0, $0, 4
    la $a0, kNamePrompt
    syscall
    addiu $v0, $0, 8
    addu $a0, $0, $s0
    addiu $a1, $0, 24
    syscall
    addiu $t9, $0, 10
    GetName_for0:
        addiu $t0, $s0, 23
    GetName_for0_check:
        blt $t0, $s0, GetName_endfor0
    GetName_for0_body:
        lbu $t1, ($t0)
        GetName_for0_body_if0:
            bne $t1, $t9, GetName_for0_body_endif0
            sb $0, ($t0)
            j GetName_endfor0
        GetName_for0_body_endif0:
    GetName_for0_inc:
        addiu $t0, $t0, -1
        j GetName_for0_check
    GetName_endfor0:

    GetName_epilogue:
        lw $s0, ($sp)
        addiu $sp, $sp, 4
        jr $ra
    GetName_endepilogue:

SetHighScore:
    SetHighScore_prologue:
        addiu $sp, $sp, -4
        sw $s0, ($sp)
    SetHighScore_endprologue:

    addiu $v0, $0, 13
    la $a0, kHighScorePath
    addiu $a1, $0, 1 # write
    syscall
    bltz $v0, ThrowOpenFileError
    addu $s0, $0, $v0
    addiu $v0, $0, 15
    addu $a0, $0, $s0
    la $a1, kNewRecordHolder
    addiu $a2, $0, 24
    syscall
    bne $v0, $a2, ThrowWriteFileError
    addiu $v0, $0, 15
    la $a1, kHighScore
    addiu $a2, $0, 8
    syscall
    bne $v0, $a2, ThrowWriteFileError
    addiu $v0, $0, 16
    syscall

    SetHighScore_epilogue:
        lw $s0, ($sp)
        addiu $sp, $sp, 4
        jr $ra
    SetHighScore_endepilogue:

SetTable:
    SetTable_prologue:
        addiu $sp, $sp, -4
        sw $s0, ($sp)
        addu $s0, $0, $a0
    SetTable_endprologue:

    SetTable_initrng:
        addiu $v0, $0, 30
        syscall
        addu $a1, $0, $a0
        lw $a0, kRngId
        syscall
    SetTable_endinitrng:

    addiu $t0, $0, 0
    addiu $t1, $0, 0
    addiu $t8, $0, 2
    addiu $t9, $0, 18
    SetTable_while0:
        bge $t1, $t9, SetTable_endwhile0
        addiu $v0, $0, 42
        lw $a0, kRngId
        addiu $a1, $0, 16
        syscall
        addu $t2, $0, $a0
        addiu $t7, $0, 1
        sllv $t7, $t7, $t2
        and $t5, $t0, $t7
        bnez $t5, SetTable_while0
        SetTable_while0_for0:
            addiu $t3, $0, 0
        SetTable_while0_for0_check:
            bge $t3, $t8, SetTable_while0_endfor0
        SetTable_while0_for0_body:
            addiu $v0, $0, 42
            lw $a0, kRngId
            addu $a1, $0, $t9
            syscall
            addu $t4, $0, $a0
            addu $t5, $s0, $t4
            lbu $t6, ($t5)
            andi $t6, $t6, 0x80
            bnez $t6, SetTable_while0_for0_body # we can skip the check because $t3 hasn't changed
            ori $t6, $t2, 0x80
            sb $t6, ($t5)
        SetTable_while0_for0_inc:
            addiu $t3, $t3, 1
            j SetTable_while0_for0_check
        SetTable_while0_endfor0:
        addu $t1, $t1, $t8
        or $t0, $t0, $t7
        j SetTable_while0
    SetTable_endwhile0:

    SetTable_epilogue:
        lw $s0, ($sp)
        addiu $sp, $sp, 4
        jr $ra
    SetTable_endepilogue:

ThrowOpenFileError:
    ThrowOpenFileError_prologue:
    ThrowOpenFileError_endprologue:

    addiu $v0, $0, 4
    la $a0, kOpenFileError
    syscall

    ThrowOpenFileError_epilogue:
        addiu $v0, $0, 17
        addiu $a0, $0, 1
        syscall
    ThrowOpenFileError_endepilogue:

ThrowReadFileError:
    ThrowReadFileError_prologue:
    ThrowReadFileError_endprologue:

    addiu $v0, $0, 4
    la $a0, kReadFileError
    syscall

    ThrowReadFileError_epilogue:
        addiu $v0, $0, 17
        addiu $a0, $0, 1
        syscall
    ThrowReadFileError_endepilogue:

ThrowWriteFileError:
    ThrowWriteFileError_prologue:
    ThrowWriteFileError_endprologue:

    addiu $v0, $0, 4
    la $a0, kWriteFileError
    syscall

    ThrowWriteFileError_epilogue:
        addiu $v0, $0, 17
        addiu $a0, $0, 1
        syscall
    ThrowWriteFileError_endepilogue:
