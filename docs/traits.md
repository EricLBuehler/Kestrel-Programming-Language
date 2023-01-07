# Traits

The functions listed use generic types. To implement traits, see [```impl```](keywords.md#impl).

- ```Add```
    - ```fn add(self: A, other: B) -> C ```
- ```Sub```
    - ```fn sub(self: A, other: B) -> C ```
- ```Mul```
    - ```fn mul(self: A, other: B) -> C ```
- ```Div```
    - ```fn div(self: A, other: B) -> C ```
- ```Pos```
    - ```fn pos(self: A) -> C ```
- ```Neg```
    - ```fn neg(self: A) -> C ```
- ```Call```
    - **Not available to implement with ```impl```**
- ```Bool```
    - ```fn bool(self: A) -> C ```
- ```Eq```
    - ```fn eq(self: A, other: B) -> C ```
- ```Ne```
    - ```fn ne(self: A, other: B) -> C ```
- ```Gt```
    - ```fn gt(self: A, other: B) -> C ```
- ```Lt```
    - ```fn lt(self: A, other: B) -> C ```
- ```Ge```
    - ```fn ge(self: A, other: B) -> C ```
- ```Le```
    - ```fn le(self: A, other: B) -> C ```