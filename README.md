A spoofing / hijacking DLL creator written in Rust.

## Overview

This tool creates a DLL that spoofs Windows system DLLs, e.g. `winmm.dll`, to trick applications into loading our version of these DLLs at application launch. By putting our code in the spoofed DLLs, we gain full control of the application process as soon as it launches.

For the details of DLL spoofing, see: https://prog.world/dll-spoofing-dll-hijacking/

## Build instructions

You need Rust nightly to build this project, since it requires some unstable features, like `asm` and `naked_functions`.

Currently, only the `x86_64-pc-windows-msvc` target is supported. Support for other targets may be added in the future.

To build the spoofing DLL in Release mode, run:

```
rustup override set nightly
cargo build --release
```

The project contains a sample code that enables HiDPI support for all applications loading the spoofing DLL. This achieves the same effect as by .exe File Properties → Compatibility → Change high DPI settings → Override high DPI scaling behavior, without writing to the registry. The sample code can be disabled by turning off the `example` feature, which makes loading the DLL a no-op.

The target DLL name to be spoofed can be changed by modifying the `libname.cfg` file in the project root. Note that some DLLs are always loaded from `System32`, like `kernel32.dll` and `user32.dll`, and therefore cannot be spoofed in this way.

## Credits

See the [original version](https://github.com/Kazurin-775/DllSpoofer) of this project, which is written in C++.
