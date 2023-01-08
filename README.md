# Kestrel Programming Language
![Minimum rustc 1.61](https://img.shields.io/badge/rustc-1.61%2B-red)
[![Pages Build Deployment](https://github.com/EricLBuehler/Kestrel-Programming-Language/actions/workflows/pages/pages-build-deployment/badge.svg)](https://github.com/EricLBuehler/Kestrel-Programming-Language/actions/workflows/pages/pages-build-deployment)

The Kestrel Programming Language is a statically and strongly typed, compiled programming language written in Rust.

## Depenendencies
See [more](https://github.com/EricLBuehler/Kestrel-Programming-Language/blob/master/Cargo.toml) details.
- ```inkwell``` (using ```llvm10-0```)

Kestrel is written in Rust, and is currently system-independent.

## Usage
See an example [here](https://github.com/EricLBuehler/Kestrel-Programming-Language/blob/master/program.ke).

```kestrel [--version | --help] [--err <error> | --warn <warning>] [<program>]```


## Links
- [Documentation](/docs/)
- [Inkwell LLVM wrapper](https://github.com/TheDan64/inkwell)
    - [Documentation](https://thedan64.github.io/inkwell/inkwell/index.html)
    - [License](https://github.com/TheDan64/inkwell/blob/master/LICENSE)