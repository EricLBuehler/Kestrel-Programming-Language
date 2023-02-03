# References

## What are references?
References, denoted by ```&```, are a way of passing information without giving ownership. They are not pointers, and modifying a reference will not change the original, referenced data. They are simply a construct for ownership rules.

## Use
References do not need to be explicitly denoted as types. Instead, a reference to a variable may be taken.
Only one function may own data. Taking references of unowned data is allowed.