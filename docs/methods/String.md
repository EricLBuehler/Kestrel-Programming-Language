# ```String``` type methods

## Instance
### ```length```
The ```length``` method returns the length of an array as a ```usize```.

```String.length(self: String) -> usize```

```
let str = String::new("Kestrel")
str.length() // == 3usize
```

## Namespace
## ```new```
The ```new``` method returns a new ```String``` from a ```char``` array. 

```String.new(arr: char[]) -> String```

```
let str = String::new("Kestrel")
```