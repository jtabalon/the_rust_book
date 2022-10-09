# Storing UTF-8 Encoded Text with Strings

## What is a String?

The `String` type is:
- growable
- mutable
- owned
- UTF-8 encoded

## Creating A New String

Many operations are shared between vectors and strings.

**Creating a New String**
```
let mut s = String::new();
```

```
fn main() {
    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();
}
```

Note: Strings are natively UTF-8 encoded.
```
fn main() {
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
}
```

`String::from` and `to_string` do the same thing, so which you choose is a matter of style and readability.

## Updating A String

You can use the following to change the contents of a string:
- `push_str` and `push`
- `+` operator
- `format!` macro

### Appending to a String with `push_str` and `push`

```
let mut s = String::from("foo");
s.push_str("bar");
``` 

`push_str` takes a **string slice** (in order to preserve ownership

```
fn main() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);
}
```

`push` adds a single **character** to the string.

```
fn main() {
    let mut s = String::from("lo");
    s.push('l');
}
```

### Concatenation with the `+` Operator or the `format!` Macro

```

fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
}
```

```
fn main() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
}
```

**Main Takeaway:**
- use `format!` macro to concatenate multiple strings

