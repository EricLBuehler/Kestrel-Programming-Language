# Traits

The functions listed use generic types. To implement traits, see [```impl```](keywords.md#impl).

## ```Add```

Binary add.

```fn add(self: A, other: B) -> C ```

## ```Sub```

Binary subtract.

```fn sub(self: A, other: B) -> C ```

## ```Mul```

Binary multiply.

```fn mul(self: A, other: B) -> C ```

## ```Div```

Binary divide.

```fn div(self: A, other: B) -> C ```

## ```Pos```

Unary positive.

```fn pos(self: A) -> C ```

## ```Neg```

Unary negative.

```fn neg(self: A) -> C ```

## ```Call```

Call function.

**Not available to implement with ```impl```**

## ```Bool```

Boolean representation.

```fn bool(self: A) -> C ```

## ```Eq```

Binary equality.

```fn eq(self: A, other: B) -> C ```

## ```Ne```

Binary inequality.

```fn ne(self: A, other: B) -> C ```

## ```Gt```

Binary greater.

```fn gt(self: A, other: B) -> C ```

## ```Lt```

Binary less.

```fn lt(self: A, other: B) -> C ```

## ```Ge```

Binary greater or equals.

```fn ge(self: A, other: B) -> C ```

## ```Le```

Binary less or equals.

```fn le(self: A, other: B) -> C ```