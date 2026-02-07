#! @title  Hello, World!
#! @author RosieTheGhostie <rosetheghost.dev@gmail.com>
#! @date   2026-02-07
#!
#! A simple program that prints "Hello, World!" to the terminal.

.data
kHello: .asciiz "Hello, World!\n"

.text
main:
  main.prologue:
  main.prologue.end:

  addiu $v0, $0, 4     # ┬ spim.print.string(kHello);
  lui $at, 0x1001      # │
  ori $a0, $at, 0x0000 # │
  syscall              # ┘

  main.epilogue:
    addiu $v0, $0, 10  # ┬ spim.system.exit();
    syscall            # ┘
  main.epilogue.end:
main.end:
