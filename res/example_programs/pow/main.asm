.data
kExponentOperator: .asciiz " ** "
kEquals: .asciiz " = "

# i haven't added support for things like `.space` and `.align` yet
.byte 0, 0, 0
x: .word 5
y: .word 3

.text
main:
    lui $at, 0x1001
    lw $a0, 0x000c($at) # lw $a0, x
    lw $a1, 0x0010($at) # lw $a1, y
    jal Pow
    addu $s0, $0, $v0
    addiu $v0, $0, 1
    lui $at, 0x1001
    lw $a0, 0x000c($at) # lw $a0, x
    syscall
    addiu $v0, $0, 4
    lui $a0, 0x1001 # la $a0, kExponentOperator
    syscall
    addiu $v0, $0, 1
    lui $at, 0x1001
    lw $a0, 0x0010($at) # lw $a0, y
    syscall
    addiu $v0, $0, 4
    lui $at, 0x1001
    ori $a0, $at, 0x0005 # la $a0, kEquals
    syscall
    addiu $v0, $0, 1
    addu $a0, $0, $s0
    syscall
    addiu $v0, $0, 11
    addiu $a0, $0, 10
    syscall

    main_epilogue:
        addiu $v0, $0, 10
        syscall
    main_endepilogue:

Pow:
    Pow_if0:
        bne $a1, $0, Pow_endif0
        addiu $v0, $0, 1
        jr $ra
    Pow_endif0:

    Pow_prologue:
        addiu $sp, $sp, -8
        sw $a1, 4($sp)
        sw $ra, 0($sp)
    Pow_endprologue:

    srl $a1, $a1, 1
    jal Pow
    lw $a1, 4($sp)
    mul $v0, $v0, $v0
    andi $t0, $a1, 1
    Pow_if1:
        beq $t0, $0, Pow_endif1
        mul $v0, $a0, $v0
    Pow_endif1:

    Pow_epilogue:
        lw $ra, 0($sp)
        addiu $sp, $sp, 8
        jr $ra
    Pow_endepilogue:
