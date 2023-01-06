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

Furthermore, the ```fn``` keyword also allows the definition of a method. A method must be tied to a struct: 

```
fn struct.name(parameter: <mutability> type, ...) -> tp {

}
fn struct.name(parameter: <mutability> type, ...) {

}
```


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