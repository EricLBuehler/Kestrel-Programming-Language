# Types

## Numeric

Note: any numeric literal may be postfixed with a specified type.

## ```i8```
The ```i8``` type is a signed 8-bit integer. This is the equivalent of ```char``` in ```C```.

## ```i16```
The ```i16``` type is a signed 16-bit integer.

## ```i32```
The ```i32``` type is a signed 32-bit integer. It is the default type for integer literals. This is the equivalent of ```int``` in ```C```.

## ```i64```
The ```i64``` type is a signed 64-bit integer. This is the equivalent of ```long``` in ```C```.

## ```i128```
The ```i128``` type is a signed 128-bit integer. This is the equivalent of ```long long``` in ```C```.

## ```u8```
The ```u8``` type is an unsigned 8-bit integer.

## ```u16```
The ```u16``` type is an unsigned 16-bit integer.

## ```u32```
The ```u32``` type is an unsigned 32-bit integer.

## ```u64```
The ```u64``` type is an unsigned 64-bit integer.

## ```u128```
The ```u128``` type is an unsigned 128-bit integer.

## ```f32```
The ```f32``` type is a 32-bit floating point number. This is the default type for floating point literals. This is the equivalent of ```float``` in ```C```.

## ```f64```
The ```f64``` type is a 64-bit floating point number. This is the equivalent of ```double``` in ```C```.

## ```usize```
The ```usize``` type is an unsigned 32-bit or 64-bit integer, depending on the compilation platform architecture.

## ```isize```
The ```isize``` type is a signed 32-bit or 64-bit integer, depending on the compilation platform architecture.

## ```char```
The ```char``` type is an unsigned 32-bit integer, and an alias for the ```u32``` type. It represents a UTF-32 codepoint.

## ```bool```
The ```bool``` type is 1-bit value. By convention, it is 1 or 0 (1 for ```true```, 0 for ```false```).

## Non-numeric

## ```void```
The ```void``` type represents data with no type information (```void``` in ```C``` or ```()``` in ```Rust```).
```
void
```

## ```fn```
The ```fn``` type represents a function pointer. Return type specification is optional for ```void``` type.
```
fn(type) -> tp
fn(type)
```

## Arrays

Arrays are designated using the ```C```-style syntax ```type[len]```
```
i8[3]
```

Array literals are given by the following ```Rust```-style syntax: ```[element, ...]```
```
[1,2,3]
```
All elements must be of the same type

# Structs

## ```String```
The ```String``` wraps an ```i8``` array with [methods](methods/String.md).

# Enums

## ```Optional```
The ```Optional``` type represents a value that may or may not be present.

```
enum Optional<T> {
    Some<T>,
    None,
}
```