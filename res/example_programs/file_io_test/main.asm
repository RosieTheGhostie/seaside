#! @title  File IO Test
#! @author RosieTheGhostie <rosetheghost.dev@gmail.com>
#! @date   2026-03-07
#!
#! A toy program that reads from and writes to text files.

.set noat

.data
kHelloPath: .asciiz "hello.txt"
kGoodbyePath: .asciiz "goodbye.txt"

.align 2
kGoodbyeWorldLength: .word 17
kGoodbyeWorld: .asciiz "Goodbye, World!\n"

kOpenFileError: .asciiz "failed to open file\n"
kReadFileError: .asciiz "failed to read file\n"

.align 2
kHelloBufferSize: .word 64
kHelloBuffer: .space 64

.text
main:
  main.prologue:
  main.prologue.end:

  jal PrintContentsOfHelloFile
  jal WriteGoodbyeWorldToFile

  main.epilogue:
    # spim.system.exit2(0);
    addiu $v0, $0, 17
    addiu $a0, $0, 0
    syscall
  main.epilogue.end:
main.end:

PrintContentsOfHelloFile:
  PrintContentsOfHelloFile.prologue:
    addiu $sp, $sp, -4
    sw $s0, 0($sp)
  PrintContentsOfHelloFile.prologue.end:

  # let fd = spim.file.open(kHelloPath, 0);
  addiu $v0, $0, 13
  lui $at, 0x1001
  ori $a0, $at, 0x0000
  addiu $a1, $0, 0
  syscall

  PrintContentsOfHelloFile.if0:
    bgez $v0, PrintContentsOfHelloFile.if0.end
  PrintContentsOfHelloFile.if0.then:
    # PrintErrorAndExit(kOpenFileError);
    lui $at, 0x1001
    ori $a0, $at, 0x002d
    jal PrintErrorAndExit
  PrintContentsOfHelloFile.if0.end:

  addu $s0, $0, $v0
  addu $a0, $0, $s0

  # let ptr = kBuffer;
  lui $at, 0x1001
  ori $a1, $at, 0x005c

  # let capacity = kBufferSize;
  lui $at, 0x1001
  lw $a2, 0x0058($at)

  PrintContentsOfHelloFile.while0.do:
    # let bytes_read = spim.file.read(fd, ptr, capacity);
    addiu $v0, $0, 14
    syscall

    PrintContentsOfHelloFile.if1:
      beq $v0, $0, PrintContentsOfHelloFile.while0.end
    PrintContentsOfHelloFile.if1.elif0:
      bgtz $v0, PrintContentsOfHelloFile.if1.else
    PrintContentsOfHelloFile.if1.elif0.then:
      # PrintErrorAndExit(kReadFileError);
      lui $at, 0x1001
      ori $a0, $at, 0x0042
      jal PrintErrorAndExit
    PrintContentsOfHelloFile.if1.else:
      # ptr += capacity; // what?
      addu $a1, $a1, $a2

      # capacity -= bytes_read;
      subu $a2, $a2, $v0
    PrintContentsOfHelloFile.if1.end:

  PrintContentsOfHelloFile.while0:
    bne $a2, $0, PrintContentsOfHelloFile.while0.do
  PrintContentsOfHelloFile.while0.end:

  # spim.print.close(fd);
  addiu $v0, $0, 16
  addu $a0, $0, $s0
  syscall

  # spim.print.string(kHelloBuffer);
  addiu $v0, $0, 4
  lui $at, 0x1001
  ori $a0, $at, 0x005c
  syscall

  PrintContentsOfHelloFile.epilogue:
    lw $s0, 0($sp)
    addiu $sp, $sp, 4
    jr $ra
  PrintContentsOfHelloFile.epilogue.end:
PrintContentsOfHelloFile.end:

WriteGoodbyeWorldToFile:
  WriteGoodbyeWorldToFile.prologue:
    addiu $sp, $sp, -4
    sw $s0, 0($sp)
  WriteGoodbyeWorldToFile.prologue.end:

  # let fd = spim.file.open(kGoodbyePath, 1);
  addiu $v0, $0, 13
  lui $at, 0x1001
  ori $a0, $at, 0x000a
  addiu $a1, $0, 1
  syscall

  WriteGoodbyeWorldToFile.if0:
    bgez $v0, WriteGoodbyeWorldToFile.if0.end
  WriteGoodbyeWorldToFile.if0.then:
    # PrintErrorAndExit(kOpenFileError);
    lui $at, 0x1001
    ori $a0, $at, 0x002d
    jal PrintErrorAndExit
  WriteGoodbyeWorldToFile.if0.end:

  addu $s0, $0, $v0

  # spim.file.write(fd, kGoodbyeWorld, kGoodbyeWorldLength);
  addiu $v0, $0, 15
  addu $a0, $0, $s0
  lui $at, 0x1001
  ori $a1, $at, 0x001c
  lui $at, 0x1001
  lw $a2, 24($at)
  syscall

  # spim.file.close(fd);
  addiu $v0, $0, 16
  addu $a0, $0, $s0
  syscall

  WriteGoodbyeWorldToFile.epilogue:
    lw $s0, 0($sp)
    addiu $sp, $sp, 4
    jr $ra
  WriteGoodbyeWorldToFile.epilogue.end:
WriteGoodbyeWorldToFile.end:

PrintErrorAndExit:
  PrintErrorAndExit.prologue:
  PrintErrorAndExit.prologue.end:

  # spim.print.string(message);
  addiu $v0, $0, 4
  syscall

  PrintErrorAndExit.epilogue:
    # spim.system.exit2(1);
    addiu $v0, $0, 17
    addiu $a0, $0, 1
    syscall
  PrintErrorAndExit.epilogue.end:
PrintErrorAndExit.end:
