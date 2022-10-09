# Hash Maps

`HashMap<K, V>` stores a mapping of types `K` to values of type `V` using a `hashing` function.

## Creating a New Hash Map

- using `new` and adding with `insert`

```
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
}
```

Hash maps store their data on the heap.

Hashmap keys and values must have the same type as each other.


