# ```String``` type methods

## Instance
### ```length```
The ```length``` instance method returns the length of an array as a ```usize```.

```String.length(self: String) -> usize```

```
let str = String::new("Kestrel")
str.length() == 3usize
```

### ```get```
The ```get``` instance method returns a reference to a character from the ```String``` array.

```String.get(self: String, index: usize) -> &u8```

```
let str = String::new("Kestrel")
str.get(1usize) == 'e' as u8
```

### ```get_array```
The ```get_array``` instance method returns a reference to the internal ```String``` array.

```String.get_array(self: String) -> &char[]```

```
let str = String::new("Kestrel")
str.get_array()
```

## Namespace
## ```new```
The ```new``` namespace method returns a new ```String``` from a ```char``` array. 

```String.new(arr: char[]) -> String```

```
let str = String::new("Kestrel")
```