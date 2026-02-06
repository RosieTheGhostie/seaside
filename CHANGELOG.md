# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- This change log.
- Issue templates.
- Dedicated installer.
- Ability to query the file path of the global configuration file via the 'path config' command.
- Feature flags to various crates.
- Type aliases `Size`, `SignedSize`, `Offset`, and `UnsignedOffset`.

### Changed

- Switch to GNU GPLv3 license.
- Reconfigure interpreter to accept a seaside executable file instead of a project directory.
- Reconfigure assembler to output a seaside executable instead of raw segments.
- Place unimplemented functionality behind a feature gate instead of commenting it out.
- Make 'disassemble' command use subcommands rather than mutually exclusive options.
- Replace 'exe-path' command with 'path binary' command.
- Rename `runtime_data` table in `memory_map.segments` to `stack_and_heap`.
- Rename `LexError` variant of `seaside_assembler::error::ParseError` to `Lex`.
- Rename `number_fmt` module in `seaside_constants` crate to `number_format`.
- Replace `seaside_assember::directives::SegmentDirective` with `seaside_constants::StaticSegment`
  and a type alias.
- Refactor project directory handling.
- Rearrange `use` statements.
- Implement string conversion for non-error types with `strum` rather than `thiserror`.
- Embed exception code in `enum` discriminant.

### Removed

- Register default value configuration via the `register_defaults` table.
- Configuration file versioning via the `version` property.
- `OutdatedVersion` variant of `seaside_error::EngineError` `enum`.
- Unused `seaside_int_utils::AllZeroes` `trait`.
- Unused `seaside-bitflags-serde` crate.

### Fixed

- Integers not being converted to floating-point numbers when assembling a floating-point value
  array.
- Confusion about "syscalls" vs. "system services".
- Readability issues throughout the project.
- Some broken paths in documentation comments.
- Unintentional hidden dependencies.
- Inappropriate compartmentalization.
- Small inconsistency in `derive` macro sorting.
- Inconsistent usage of type aliases.
- Unidiomatic type aliases.
- Incorrect type for service code in `ServiceUnimplemented` variant of
  `seaside_interpreter::SyscallFailureKind`.

## [1.3.0] - 2025-07-13

### Added

- MIT license.
- Rudimentary documentation of unsupported instructions (#12).
- Contextual information to assembler errors.
- Dedicated `enum`s for register indices.
- Ability to parse unimplemented assembler directives.

### Changed

- Relocate 'Seaside.toml' to OS-specified directory.
- Redesign system service configuration to enable custom mappings (#8).
- Compartmentalize branches of engine into crates (#10).
- Refactor coprocessor 1 instruction processing (#14).
- Rework register file interface.

### Removed

- Assembler directive configuration via the `features.assembler.directives` property.

### Fixed

- Incorrect opcode for `sc` (#13).
- Incorrect names for `movt` and `movf` when disassembled.
- Disassembler not treating `$rd` as a coprocessor 0 register in `mfc0` and `mtc0` instructions.
- Disassembler not treating `$ft` as a coprocessor 1 register in `lwc1`, `ldc1`, `swc1`, and `sdc1`
  instructions.
- Disassembler ignoring the `$rd` in `jalr` instructions.

## [1.2.0] - 2025-01-30

### Added

- MIPS assembler accessible via the 'assemble' command.
- Program arguments via `argc` and `argv`.
- `"spim.system.sbrk"` system service.
- Option to enable freeing heap memory with `"spim.system.sbrk"`.
- More crash handler context for syscall exceptions (#6).

### Fixed

- Lack of implementation for `blez` (#4).
- `teqi` and `tnei` instructions not sign-extending the second operand prior to the comparison.
- Accidental presence of 'experiment' command in production builds (#5).
- Incorrect implementation of offset-to-address conversion in disassembler.
- Engine panicking when requesting an unimplemented system service.
- Incorrect arrangement of `$fs` and `$fd` fields in coprocessor 1 format documentation.

## [1.1.0] - 2025-01-12

### Added

- MIPS32 disassembler accessible via the 'disassemble' command.
- Logo.
- Banner image.

## [1.0.3] - 2025-01-09

### Added

- Build for GNU toolchain on Windows x86_64 platforms.
- WIP documentation for the MIPS32 ISA in the GitHub repository's wiki.

### Fixed

- Miscellaneous documentation oversights.
- Inconsistent conversions from pascal case to snake case.
- Broken implementation of system service `"spim.read.char"` (#1).

## [1.0.2] - 2025-01-05

### Added

- System service documentation in the GitHub repository's wiki.

### Fixed

- Unintentional exclusion of some service codes in implementation of `syscall` instruction.
- System service `"spim.read.string"` incorrectly appending an extra newline character on Unix-based
  operating systems.
- Incorrect naming of some local variables.

## [1.0.1] - 2025-01-04

### Added

- 'Seaside.toml' documentation in the GitHub repository's wiki.

### Fixed

- Certain properties mistakenly being marked as required in configuration files.
- Broken link to `Interpreter` struct in `MemoryMap` documentation.

## [1.0.0] - 2025-01-04

### Added

- MIPS32 interpreter accessible via the 'run' command.
- Implementations for 100+ instructions.
- Support for 26 of MARS' 39 system services.
- Ability to configure engine via 'Seaside.toml'.

[Unreleased]: https://github.com/RosieTheGhostie/seaside/compare/v1.3.0...HEAD
[1.3.0]: https://github.com/RosieTheGhostie/seaside/compare/v1.2.0...v1.3.0
[1.2.0]: https://github.com/RosieTheGhostie/seaside/compare/v1.1.0...v1.2.0
[1.1.0]: https://github.com/RosieTheGhostie/seaside/compare/v1.0.3...v1.1.0
[1.0.3]: https://github.com/RosieTheGhostie/seaside/compare/v1.0.2...v1.0.3
[1.0.2]: https://github.com/RosieTheGhostie/seaside/compare/v1.0.1...v1.0.2
[1.0.1]: https://github.com/RosieTheGhostie/seaside/compare/v1.0.0...v1.0.1
[1.0.0]: https://github.com/RosieTheGhostie/seaside/releases/tag/v1.0.0
