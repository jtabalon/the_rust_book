# The Slice Type

``` rust

let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];

```

- If you want to start at index zero, you can drop the value before the two periods.

``` rust
let s = String::from("hello");

let slice = &s[0..2];
let slice = &s[..2];
```

- You can also drop the trailing number.

``` rust
let s = String::from("hello");

let len = s.len();

let slice = &s[3..len];
let slice = &s[3..];
```

- To index a whole string, you can just use the `..` operator without leading or trailing numbers.

``` rust
let s = String::from("hello");

let len = s.len();

let slice = &s[0..len];
let slice = &s[..];
```

## String Literals are slices.

``` rust
let s = "Hello, world!";
```

`s` is of type `&str` - a slice pointing to that specific point of the binary. String literals are immuable.

