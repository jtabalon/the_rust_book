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

## Hash Maps and Ownership

For types that implement `Copy` trait, values are copied into the hash map

For owned values like `String`, values are moved and the hashmap becomes the owner of those values.

```
fn main() {
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
}
```

## Updating a Hash Map
There can only be one value associated with a key.

### Overwriting a Value
If the key is present, the most recent value is returned.

```
fn main() {
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
}
```

### Adding a Key and Value Only if a Key Isn't Present

Hash maps have a special API called `entry` that takes a key as a parameter
- return value is enum called `Entry` that represents a value that might or might not exist.

```
fn main() {
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
}
```

`or insert` returns mutable reference to the value if key exists, and inserts parameter as new value for the key and returns mutable reference to the new value.
- cleaner than writing logic manually, and plays more nicely with borrow checker.

### Update Value based on the Old Value

```
fn main() {
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
}
```

The code above counts the number of occurrences of each word in the string.


## Summary
Hashmaps, strings, and vectors are all important data structures.









