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

## String Slices as Parameters

``` rust
fn first_word(s: &String) -> &str {
```

Prefer the bottom one to the signature above, to allow use on both `&String` values as well as `&str` values.

``` rust
fn first_word(s: &str) -> &str {
```

Defining function to take string slice instead of reference makes a function more general and useful without losing any functionality.

## Other Slices

``` rust

#![allow(unused)]
fn main() {
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];

assert_eq!(slice, &[2, 3]);
}
```

- Slices store a reference to the first eleement and a length.

