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
arr.get(0usize)
```

### ```set```
The ```set``` instance method sets the item of type ```T``` (array element type) at a specified index, and returns an ```Optional```.

```Array.set(self: Array, index: usize, item: T) -> Optional<void>```

```
let arr = [1,2,3]
arr.set(0usize, 0)
```