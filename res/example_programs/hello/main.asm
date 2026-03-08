#! @title  Hello, World!
#! @author RosieTheGhostie <rosetheghost.dev@gmail.com>
#! @date   2026-03-07
#!
#! A simple program that prints "Hello, World!" to the terminal.

.set noat

.data
kHello: .asciiz "Hello, World!\n"

.text
main:
  main.prologue:
  main.prologue.end:

  # spim.print.string(kHello);
  addiu $v0, $0, 4
  lui $at, 0x1001
  ori $a0, $at, 0x0000
  syscall

  main.epilogue:
    # spim.system.exit();
    addiu $v0, $0, 10
    syscall
  main.epilogue.end:
main.end:
