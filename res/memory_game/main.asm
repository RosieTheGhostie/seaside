.data
kFlippedOver:
	.byte 0x1b
	.asciiz "[90m"
kAnsiReset:
	.byte 0x1b
	.asciiz "[0m"
kTableHeader: .asciiz "   0  1  2  3  4  5\n"
kRngId: .word 69

.text
main:
    main_prologue:
        addiu $sp, $sp, -20
    main_endprologue:

    addu $a0, $0, $sp
    jal SetTable
    lbu $t0, ($sp)
    ori $t0, $t0, 0x40
    sb $t0, ($sp)
    addu $a0, $0, $sp
    jal DisplayTable

    main_epilogue:
        addiu $sp, $sp, 20
        addiu $v0, $0, 10
        syscall
    main_endepliogue:

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
        addiu $at, $0, 6
        bge $s2, $at, DisplayTableRow_endfor0
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
        addiu $at, $0, 6
        bge $s2, $at, DisplayTableRow_endfor1
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
        addiu $at, $0, 6
        bge $s2, $at, DisplayTableRow_endfor2
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
