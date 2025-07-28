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
- [ ] Support for `.eqv`

### Interpreter (*mostly implemented*)

Run programs seamlessly with the power of the seaside interpreter.

- [X] 136/136 basic instructions supported by MARS
- [ ] 27/39 system services supported by MARS
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
- [X] Self-modifying code[^1]
- [X] Delay slot[^1]
- [X] System service mapping
- [X] Ability to free heap allocations
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

When installing seaside, you have two options: run the installer, or compile from source.

### Option A. Using the Installer (recommended)

The [seaside installer](https://github.com/RosieTheGhostie/seaside-installer) streamlines the process of installing, updating, and uninstalling seaside. It handles all the OS-dependent stuff for you, making for a much simpler and safer experience.

Start by downloading the [latest release](https://github.com/RosieTheGhostie/seaside-installer/releases/latest) of the seaside installer. Newer versions of the installer don't necessarily allow you to install newer/older versions of seaside, but they may be less buggy and have more features.

Once the installer is on your system, navigate to the directory it's in via the terminal and run it. This should print the installer's usage. From this point, run whichever command best fits your needs.

### Option B. Compiling from Source

If you wish to compile seaside yourself (for some reason), that is an option, too; however, it will require much more setup to get working.

#### Prerequisites

As with any other Rust app, you'll need [cargo](https://doc.rust-lang.org/cargo/) to compile it. I won't go into detail on how to install that here, but you can find some information about it [here](https://doc.rust-lang.org/cargo/getting-started/installation.html).

Other than that, seaside shouldn't need anything else to be installed on your computer.

#### Compilation

1. Navigate to seaside's root directory. This is the directory containing 'Cargo.toml'.
2. In your terminal/command prompt of choice, enter the following command:
    ```bash
    cargo build -r
    ```
3. Sit back and watch the Rust compiler do its magic.

At this point, you should find the seaside executable in './target/release'. You may delete all other files in './target' if you wish, but leaving them around will speed up future compilations.

#### Putting Stuff in the Right Directories

For the best experience, it is crucial to put files in the right place.

##### 'Seaside.toml'

seaside expects its global configuration file to be in a very specific location on your computer. This location is (like most other things going forward) OS-dependent.

- **Linux:** /home/*user*/.config/seaside/Seaside.toml
- **Windows:** C:\\Users\\*user*\\AppData\\Roaming\\seaside\\config\\Seaside.toml

Replace *user* with your username.

I recommend copying 'Seaside.toml' from the 'res' directory of this repository to the correct location for your machine.

##### The seaside Executable

While not technically required, having seaside in a specific location can expedite its use later on.

- **Linux:** /usr/local/bin/seaside
- **Windows:** C:\\ProgramData\\seaside\\seaside.exe

> [!NOTE]
> 'ProgramData' is a hidden folder on Windows.

On Linux, this step makes it possible to run seaside anywhere on your computer without referring to it by path. Windows, on the other hand, requires a bit more setup to achieve this.

#### Adding seaside to 'Path' (Windows only)

Start by pulling up the start menu (most easily done by pressing the \[Win\] key). Type in "path", which should bring up something that says "Edit the system environment variables". If so, open it and click the button that says "Environment Variables".

> [!TIP]
> If that didn't work for whatever reason, it's not too hard to go there yourself.
>
> 1. Open up Control Panel
> 2. In the search bar, type in "environment"
> 3. Under "System", click "Edit environment variables for your account"

Scrolling through the user variables, you should find one named "Path". This is a list of directories Windows will search whenever a command is executed. Double-click on it to open the editing interface.

> [!NOTE]
> This variable exists on Linux, too, but we didn't need to mess with it there because we used a directory that's already in the path.

Click the "New" button and type in "C:\\ProgramData\\seaside". Once you press \[Enter\], click the "OK" buttons on both windows to commit your changes.

### Configuration

seaside has extensive configuration options, as described [above](#configuration-mostly-implemented). These are designed to mimic MARS' default settings out of the box, but you are welcome to change them however you see fit.

If seaside is ran in a directory with a file named 'Seaside.toml', that file will be used to set up the engine. Otherwise, it will use the global 'Seaside.toml'.

Extensive documentation of the available configuration options can be found [here](https://github.com/RosieTheGhostie/seaside/wiki/Configuration-Manual).

[^1]: The option exists in the config, but it doesn't do anything yet.
