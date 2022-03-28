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
- Adding data: "push onto stack"
- Removing data: "pop off the stack"
- Pushing to stack is faster than allocating on heap
    - don't have to search for place to store new data (always at top of stack)
- Analogy: 
    - Think plates (First in, Last Out)
- 


*Heaps*
- not as organized as stacks
- Putting data on the heap:
    - request certain amount of space
    - memory allocator finds spot in heap that suffices and marks it as in use
    - returns a pointer
- Analogy:
    - Going to a restaurant and asking to be seated given a party size of n.
    - Staff seats you
    - If someone arrives late, they ask the staff
- **slower** than accessing data on the stack because you have to follow a pointer to get there
- Analogy: 
    - inefficient to take orders of one person at table A, then one person at table B, then back to table A

When code calls a function, values are passed into the function, and local variables pushed onto the stack.
- When funcction is over, values get popped off the stack.

Ownership
- understanding what data on the heap
- minimize amount of duplicate data on heap
- cleaning up unused data on the heap