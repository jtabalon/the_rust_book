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

## Accessing Values in a Hash Map

- use the `get` method

```
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
}
```

`get` returns an `Option<&V>`. If there's no value for the key, `get` will return `None`.

We handle the `Option` by calling `copied` to get Option<i32> rather than Option<&i32>, then `unwrap_or` to set `score` to zero if `scores` doesn't have an entry for the key.

We can iterate through each key/value pair using a `for` loop.

```
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}
```



