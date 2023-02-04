# References

## What are references?
References, denoted by ```&``` or ```&mut```, are a way of passing information without giving ownership. Instead of passing the actual data, a pointer is passed instead.

## Use
A reference to a variable may be taken. Only one function may own data. Taking references of unowned data is not allowed. Struct members may not be references, as this introduces complexity.

## Mutable references
Mutable references, denoted by ```&mut``` are a way of passing a reference to data that may be mutated. When mutable references are assigned to directly, their underlying data changes. If their attributes are mutated, then the underlying data also changes.