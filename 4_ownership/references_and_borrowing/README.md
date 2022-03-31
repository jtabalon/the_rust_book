# 4.2 References + Borrowing

& denotes references - allow to refer to some value without taking ownership of it

- Creating a reference means we're borrowing.
- if we try to modify something we're borrowing, Rust throws an error.

## Mutable References

We can modify a borrowed value by using a mutable reference instead.

NOTE: You can only have one mutable reference to a piece of data at a time.

NOTE: You also cannot have a mutable reference when there is an immutable one to the same value.

A **reference's** scope starts from where it is introduced until the last time the reference is used.

## Dangling pointers

Dangling Pointer: pointer that references a location in memory that may have been given to someone else by freeing some memory while preserving a pointer to that memory.



