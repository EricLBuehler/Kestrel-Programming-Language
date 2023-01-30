# Kestrel Programming Language
![Minimum rustc 1.61](https://img.shields.io/badge/rustc-1.61%2B-red)
[![Pages Build Deployment](https://github.com/EricLBuehler/Kestrel-Programming-Language/actions/workflows/pages/pages-build-deployment/badge.svg)](https://github.com/EricLBuehler/Kestrel-Programming-Language/actions/workflows/pages/pages-build-deployment)
[![MIT License](https://img.shields.io/badge/License-MIT-yellow)](https://github.com/EricLBuehler/Kestrel-Programming-Language/blob/master/LICENSE)

The Kestrel Programming Language is a multi-paradigm, statically and strongly typed, compiled programming language written in Rust.

## Features
- **Ahead of time compilation** - Kestrel is compiled ahead of time (AOT), instead of being interpreted or JIT compiled. AOT compilation allows Kestrel to catch entire classes of runtime errors, vastly improving the developer experience.

- **Statically typed** - Kestrel resolves types at compile time, resulting in immediate warnings and feedback.

- **Performance** - AOT compilation means that Kestrel programs are compiled directly to machine code, allowing programs to be executed on any target platform natively, with blazing fast performance.

- **Helpful compiler** - Descriptive and detailed error messages improve the debugging experience.

- **Memory safe** - Data ownership and [references](https://github.com/EricLBuehler/Kestrel-Programming-Language/blob/master/docs/references.md) enforce what are good practices in other languages and eliminate an entire class of errors.

## Depenendencies
See [more](https://github.com/EricLBuehler/Kestrel-Programming-Language/blob/master/Cargo.toml) details.
- ```inkwell``` (using ```llvm10-0```)

Kestrel is written in Rust, and is currently system-independent.

## Usage
See an example [here](https://github.com/EricLBuehler/Kestrel-Programming-Language/blob/master/program.ke).

```kestrel [--version | --help] [--err <error> | --warn <warning>] [<program>]```


## Links
- [Documentation](https://github.com/EricLBuehler/Kestrel-Programming-Language/tree/master/docs/)
- [GitHub Pages](https://ericlbuehler.github.io/Kestrel-Programming-Language/)
- [Inkwell LLVM wrapper](https://github.com/TheDan64/inkwell)
    - [Documentation](https://thedan64.github.io/inkwell/inkwell/index.html)
    - [License](https://github.com/TheDan64/inkwell/blob/master/LICENSE)