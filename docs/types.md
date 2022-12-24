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

## Non-numeric

## ```void```
The ```void``` type represents data with no type information (```void``` in ```C``` or ```()``` in ```Rust```).
```
void
```

## ```fn```
The ```fn``` type represents a function pointer. Return type specification is optional.
```
fn(type) -> tp
fn(type)
```