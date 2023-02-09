# ```out```

The ```out``` struct defines methods for outputting data.

### ```print```
The ```print``` method prints the value contained in a string to ```stdout```. The value returned is negative to signify an error.

```fn std::out::print(str: String) -> i32```

```
std::out::print(String::new("Hello, Kestrel!\n"));
```