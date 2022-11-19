# Enums and Pattern Matching

Enums allow you to define a type by enumerating its possible variants.

Rust's enums are most similar to `algebraic data types` in functional languages such as F#, OCaml, and Haskell.

## IP Address Example


``` rust

enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}

fn route(ip_kind: IpAddrKind) {}
```

This is helpful because both values of IpAddrKind are of the same type.

You can define a function that takes *any* IpAddrKind

We can attach data to a variant of an enum directly.


``` rust
fn main() {
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));
}
```

You automatically get a constructor function defined as a result of defining an enum.

Another advantage versus structs: Each variant can have different types and amounts of associated data.

``` rust
fn main() {
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
}
```

STD library has a IP address store and encoder.

``` rust

#![allow(unused)]
fn main() {
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
}

```

