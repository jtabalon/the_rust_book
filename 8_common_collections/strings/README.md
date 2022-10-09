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
