# Unrecoverable Errors with `panic!`

Two ways to cause panic in practice:
- action that causes code to panic
- explicitly calling `panic!`

## Using a `panic!` Backtrace

**Example**

Following code tries to access out of bounds:

```{rust}
fn main() {
    let v = vec![1, 2, 3];

    v[99];
}
```

Aside: In C, attempting to read out of bounds is undefined behavior. You may experience a *buffer overread* and may lead to security vulnerabilities if attacker is able to manipulate the index.

`RUST_BACKTRACE=1` allows us to get a backgrace of exactly what happened to cause the error. 


The key to reading the backtrace is to start from the top and read until you see the files you wrote.

