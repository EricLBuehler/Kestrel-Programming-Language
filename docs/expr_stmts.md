# Expressions and Statements

Expressions and statements are the building blocks of kestrel programs.

## Statements
Keywords such as ```impl``` that appear at global scope are statements. However, statements that appear in a function body (not global scope), may be implemented by appending ```;``` after an expression.

## Expressions
Expressions express values, and actions. All calls, operations, and some control flow are expressions. Expressions may be converted into expressions by appending ```;```. This causes them to evaluate to ```void```.