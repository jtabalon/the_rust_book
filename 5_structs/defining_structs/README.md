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

`..user1` must come last to specify remaining fields.

Note: you cannot use user1 after instantiating user2, because String from user1 was moved to user2. This would not be the case if new String values were used for both email and username.


## Tuple Structs

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

Tuple structs do not have names associated with their fields, but have types of fields.

## Unit Like Structs Without Any Fields

Unit like structs behave similarly to `()`.

struct AlwaysEqual;

```rust
fn main() {
    let subject = AlwaysEqual;
}
```

These are useful when we want to implement a trait, but don't have data to store in the type itself.

IT's possible for structs to store references to data owned by something else, but requires the use of *lifetimes*.

**Lifetimes** ensure data references by a struct is valid for as long as the strcut is.






