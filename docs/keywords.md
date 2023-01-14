# Keywords

## ```let```
The ```let``` keyword defines a variable.

```
let <mutability> name: type = value
```


## ```fn```
The ```fn``` keyword defines a function. If no ```return``` statement is executed, the last expression will be returned. Return type specification is optional for ```void``` type.

```
fn name(parameter: <mutability> type, ...) -> tp {
    ...
}
fn name(parameter: <mutability> type, ...) {
    ...
}
```

Furthermore, the ```fn``` keyword also allows the definition of an instance method. A method must be tied to a struct: 

```
fn struct.name(parameter: <mutability> type, ...) -> tp {

}
fn struct.name(parameter: <mutability> type, ...) {

}
```

Additionally, the ```fn``` keyword also allows the definition of namespace methods. A method must be tied to a struct: 

```
fn struct::name(parameter: <mutability> type, ...) -> tp {

}
fn struct::name(parameter: <mutability> type, ...) {

}
```

Functions may be templated. Specify template types using angle brackets after the name. The template types may be used like any other type in the function signature.

```
fn name<T>(parameter: <mutability> type, ...) -> T {

}
fn name<T>(parameter: <mutability> type, ...) {

}
```


See more documentation on functions [here](functions.md).


## ```mut```
The ```mut``` keyword defines the parameter or a variable as mutable.

```
fn name(parameter: <mutability> type, ...) -> tp {
    ...
}

let mut name: type
```

## ```return```
The ```return``` keyword returns data from a function.

```
fn name(parameter: <mutability> type, ...) -> tp {
    ...
    return data
}
```

## ```as```
The ```as``` keyword converts one primitive data type to another, with **no** overflow checking. This is similar to casting in ```C```.

```
1000u32 as i8
```

## ```struct```
The ```struct``` keyword allows the definition of a typed struct.

```
struct name {
    member: type
}
```

Fine-grained, per-member mutablility control is also allowed:

```
struct name {
    <mutability> member: type
}
```

## ```impl```
The ```impl``` keyword allows for the implementation of traits onto structs.

```
impl trait for struct {
    ...
}
```

The body of the ```impl``` statement must be a function according to [traits.md](traits.md).

## ```if```, ```elif```, ```else```
The ```if```, ```elif```, and ```else``` keywords implement control flow.

```
if condition {
    ...
}
elif condition2 {
    ...
}
else {
    ...
}
```

All conditions must be of type [```bool```](types.md#bool).

## ```loop```
The ```loop``` keyword executes the given code infinitely.

```
loop {
    ...
}
```

## ```break```
The ```break``` keyword moves the control flow out of the loop.

## ```continue```
The ```continue``` keyword moves the control flow to the beginning of the loop.

## ```while```
The ```while``` keyword executes the given code while a specified expression is true.

```
while expression {
    ...
}
```

## ```enum```
The ```enum``` keyword creates an ```enum``` type.

```
enum {
    ...
}
```

Each line contains a name for a variant of the ```enum```.
Access each variant using the [```::```](symbols.md#namespace-attribute-access--assignment).