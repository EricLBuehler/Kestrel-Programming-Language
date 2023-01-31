# Function Types

Functions are defined by the [```fn```](keywords.md#fn) keyword. There are 2 variants:

- Methods
- Namespace

## Methods
Methods are linked to a struct instance and take a ```self``` argument of the type of the specified struct.

## Namespace functions
Namespace functions are, like methods, connected to structs, but do not take a ```self``` argument.

## Generics
Generic functions may be used as methods or namespace functions, but they allow for static dispatch of template functions with the appropriate types when called. [Traits](keywords.md#trait) that are used with [```dyn```](keywords.md#dyn) may not contain any generic functions.