# `seaside`: A modern tool to learn MIPS Assembly

This tool rethinks the way educators and students write MIPS Assembly.

## Motivation

As of 2025, the gold standard for MIPS in education is [MARS](https://github.com/gon1332/mars). MARS is undoubtedly a wonderful piece of software, but it hasn't seen a major update in nearly a decade, and having a separate IDE for developing MIPS Assembly code is rather inconvenient.

The other major motivation for making `seaside` is that I, Rose Thorne, needed a Capstone project to graduate. I took a course on computer architecture during the Autumn 2024 semester, and seeing how my project needs to connect to a class I've taken, this seems like a good candidate.

## Features

`seaside` is currently a huge work-in-progress, so there aren't many features yet; however, I aim to make this as good (if not better) than MARS in the coming years.

### Assembler (*not implemented*)

- [ ] Multi-file support
- [ ] MARS-style macros
- [ ] Support for `.set`

### [Debug Adapter Protocol (DAP)](https://microsoft.github.io/debug-adapter-protocol/) Compatibility (*not implemented*)

- [ ] Set breakpoints
- [ ] Step through a program
- [ ] Step over a procedure call
- [ ] Inspect the contents of each register
- [ ] View virtual memory in real-time

### Interpreter (*partially implemented*)

- [X] 134/134 basic instructions supported by MARS
- [ ] 25/39 system calls supported by MARS
  - [X] 8/8 printing services
  - [X] 5/5 user input services
  - [ ] 3/4 file services
  - [ ] 4/7 misc. system services
  - [X] 5/5 RNG services
  - [ ] 0/10 GUI services
- [X] Control over user- and kernel-space

### Configuration (*mostly implemented*)

- [X] Byte order (a.k.a. endianness)
- [X] Memory map
- [X] Allocated memory for each region
- [X] Default values of registers
- [X] Crash handler
- [X] Current working directory for file IO
- [X] Enable/disable self-modifying code
- [ ] Enable/disable individual system calls and directives
- [ ] + more!!!

## Installation

Seeing as how the project is still in its infancy, there is no release available yet. You can compile the code yourself with `cargo`, but I won't make an effort to write a proper guide until later.
