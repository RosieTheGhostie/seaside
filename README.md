![seaside: a modern tool to learn MIPS assembly](res/banner.png)

## Motivation

As of 2025, the gold standard for MIPS in education is [MARS](https://github.com/gon1332/mars). MARS is undoubtedly a wonderful piece of software, but it hasn't seen a major update in nearly a decade, and having a separate IDE for developing MIPS Assembly code is rather inconvenient.

The other major motivation for making seaside is that I, Rose Thorne, needed a Capstone project to graduate. I took a course on computer architecture during the Autumn 2024 semester, and seeing how my project needs to connect to a class I've taken, this seems like a good candidate.

## Features

seaside is currently a huge work-in-progress, so there aren't many features yet; however, I aim to make this as good (if not better) than MARS in the coming years.

### Assembler (*partially implemented*)

Assemble MIPS source code into raw binary files for execution.

- [X] Single-file support
- [ ] Multi-file support
- [X] Basic instruction formats
- [ ] Pseudo-instructions and extended formats
- [ ] MARS-style macros
- [ ] Support for `.set`

### Interpreter (*mostly implemented*)

Run programs seamlessly with the power of the seaside interpreter.

- [X] 136/136 basic instructions supported by MARS
- [ ] 26/39 system calls supported by MARS
  - [X] 8/8 printing services
  - [X] 5/5 user input services
  - [X] 4/4 file services
  - [ ] 5/7 misc. system services
  - [X] 5/5 RNG services
  - [ ] 0/10 GUI services
- [X] Control over user- and kernel-space

### Disassembler

Convert raw machine code into a human-readable representation using MIPS Assembly.

- Single instruction disassembly
- Full segment disassembly

### Configuration (*mostly implemented*)

Change the behavior of the seaside engine to your preference.

- [X] Byte order (a.k.a. endianness)
- [X] Memory map
- [X] Allocated memory for each region
- [X] Default values of registers
- [X] Crash handler
- [X] Current working directory for file IO
- [X] Toggle self-modifying code
- [X] Toggle individual system calls
- [X] Toggle individual directives
- [ ] + more!!!

### [Debug Adapter Protocol (DAP)](https://microsoft.github.io/debug-adapter-protocol/) Compatibility (*not implemented*)

Use with a complementary extension in your IDE to debug your programs.

- [ ] Set breakpoints
- [ ] Step through a program
- [ ] Step over a procedure call
- [ ] Inspect the contents of each register
- [ ] View virtual memory in real-time

## Usage

When running a program, the seaside engine expects you to provide a directory with the following files:

- 'text': MIPS machine code
- 'extern' (optional): binary data
- 'data' (optional): binary data
- 'ktext' (optional): MIPS machine code
- 'kdata' (optional): binary data

Each of these files directly corresponds to their respective segments in the MIPS assembly code.

As of v1.2, seaside can assemble programs itself, so external tools like MARS are unnecessary; however, seaside's assembler is in its infancy and therefore lacks some features. If you want to use pseudo-instructions, macros, multiple source files, etc., MARS may be a better choice for the time being.

## Installation

I plan to make this process more streamlined at some point (maybe with package managers or a dedicated installer), but for now, it's a bit complicated. If you have ideas for how to improve the installation process, *please* let me know.

### Prerequisites

As with any other Rust app, you'll need [cargo](https://doc.rust-lang.org/cargo/) to compile it. I won't go into detail on how to install that here, but you can find some information about it [here](https://doc.rust-lang.org/cargo/getting-started/installation.html).

Other than that, seaside shouldn't need anything else to be installed on your computer.

### Compilation

1. Navigate to seaside's root directory. This is the directory containing 'Cargo.toml'.
2. In your terminal/command prompt of choice, enter the following command:
    ```bash
    cargo build --release
    ```
3. Sit back and watch the Rust compiler do its magic.

> [!NOTE]
> On Linux, cargo demands that you append `-Znext-lockfile-bump` to the build command. No idea why.

At this point, you should find the seaside executable in './target/release'. You may delete all other files in './target' if you wish, but leaving them around will speed up future compilations.

## Post-Installation

Although seaside will technically work now, there are some things I would recommend doing to get the best experience.

### Adding seaside to the System Path

To avoid typing out the full executable path every time you want to run seaside, you can add it to your system path. This process will vary wildly by operating system, so I won't attempt to describe it here.

Regardless of your OS, the directory you'll append to your system path will be the parent of the seaside executable. The easiest way to find this is to run the following command from the seaside root directory:

```bash
cargo run -- exe-path
```

This will print the absolute path of the executable. Simply remove the executable from the end of that path to get the parent's path.

### Configuration

seaside has extensive configuration options, as described [above](#configuration-mostly-implemented). These are designed to mimic MARS' default settings out of the box, but you are welcome to change them however you see fit.

If seaside is ran in a directory with a file named 'Seaside.toml', that file will be used to set up the engine. Otherwise, it will attempt to find 'Seaside.toml' in its own root directory. For this reason, you should never move or delete the configuration file shipped with seaside.

Extensive documentation of the available configuration options can be found [here](https://github.com/RosieTheGhostie/seaside/wiki/Configuration-Manual).
