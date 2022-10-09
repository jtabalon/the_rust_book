# Storing Lists of Values with Vectors

`Vec<T>` &rarr; Vector

Vectors allow you to store more than one value in a single data structure that puts all the values next to each other in memory.

## Creating a New Vector

```
let v: Vec<i32> = Vec::new();
```

You can create a `Vec<T>` with intiall values where Rust can infer the type of value you want to store.

```
let v = vec![1,2,3];
```

## Updating a Vector

To add elements to a vector, we can use the `push` method:

```
let mut v = Vec::new();
v.push(5)
v.push(6)
v.push(7)
v.push(8)
```

## Reading Elements of Vectors

You can reference values stored in a vector using the following:
- indexing 
- `get` method

```{rust}
let v = vec![1, 2, 3, 4, 5];
let third: &i32 = &v[2];

let third: Option<&i32> = v.get(2);

match third {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}
```

In the case above, both return the same value.









