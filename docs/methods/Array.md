# ```Array``` type methods

## Instance
### ```length```
The ```length``` instance method returns the length of an array as a ```usize```.

```Array.length(self: Array) -> usize```

```
let arr = [1,2,3]
arr.length() == 3usize
```

### ```get```
The ```get``` instance method returns an ```Optional``` item of type ```T``` (array element type) from the array.

```Array.get(self: Array, index: usize) -> Optional<T>```

```
let arr = [1,2,3]
arr.get(0usize) == 1
```