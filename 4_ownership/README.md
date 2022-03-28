# 4. Ownership

Ownership is Rust's most unique feature.

Ownership enables Rust to make memory safety guarantees without a garbage collector.

## What is ownership?

Ownership is a set of rules that governs how a Rust program manages memory.

Different memory management approaches:
- Garbage collection: looks for no-longer used memory as the program runs
- explicitly allocate and free memory
- Rust: managed by a system of ownership with rules that the compiler checks
    - if any of the rules are violated, the program won't compile



### Stacks + Heaps

- Necessary to understand for memory management

*Stacks*
- Think plates (First in, Last Out)
- Adding data: "push onto stack"
- Removing data: "pop off the stack"

*Heaps*
- not as organized as stacks
- **slower** than accessing data on the stack because you have to follow a pointer to get there
- 