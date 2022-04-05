 # Defining Structs


Structs are similar to tuples.

Structs name should describe significance of data being grouped together.

Names and types of pieces of data are called **fields**.

```rust

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

```

To use a struct, create an instance of the struct by defining values for each of the fields.

Make use of `key:value` pairs
- keys: names of field
- values: data we want stored in the fields

``` rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
}
```

You can change a value by using dot notation and assigning into a particular field... **if the instance is mutable**.

We are not allowed to mark only certain fields as mutable.


``` rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
}

```


## Creating Instances From Other Instances with Struct Update Syntax

``` rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // --snip--

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
}
```

You can use struct update to create a new instance without code.

``` rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // --snip--

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}

```