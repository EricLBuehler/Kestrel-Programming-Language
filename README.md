# Kestrel Programming Language
![Minimum rustc 1.61](https://img.shields.io/badge/rustc-1.61%2B-red)

The Kestrel Programming Language is a statically and strongly typed, compiled programming language written in Rust.

## Depenendencies
See [more](Cargo.toml) details.
- ```inkwell``` (using ```llvm10-0```)

Kestrel is written in Rust, and is currently system-independent.

## Usage
See an example [here](program.ke).

```kestrel [--version | --help] [--err <error> | --warn <warning>] [<program>]```


## Links
- [Documentation](/docs/)
- [Inkwell LLVM wrapper](https://github.com/TheDan64/inkwell)
    - [Documentation](https://thedan64.github.io/inkwell/inkwell/index.html)
    - [License](https://github.com/TheDan64/inkwell/blob/master/LICENSE)