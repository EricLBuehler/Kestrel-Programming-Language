# Keywords

## ```let```
The ```let``` keyword defines a variable.

```
let <mutability> name: type = value
```


## ```fn```
The ```fn``` keyword defines a function. If no ```return``` statement is executed, the last expression will be returned. Return type specification is optional.

```
fn f(name: <mutability> type, ...) -> tp {
    ...
}
```

## ```mut```
The ```mut``` keyword defines the parameter or a variable as mutable.

```
fn f(name: mut type, ...) -> tp {
    ...
}

let mut name: type
```

## ```return```
The ```return``` keyword returns data from a function.

```
fn f(name: mut type, ...) -> tp{
    ...
    return data
}
```

