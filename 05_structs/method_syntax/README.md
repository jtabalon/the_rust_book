# Method Syntax

Unlike functions, methods are defined within the context of a struct (or an enum or a trait object).





## Defining Methods

``` rust

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

```

`impl` (implementation) block associates with the Rectangle type.

`&self` is short for `self: &Self`. 

Methods can take ownership of `self`, borrow `self` immutably, or borrow `self` mutably.

Having a method that takes ownership of the instance by using just `self` as the first parameter is rare.

## Main reason for methods.

The main reason for using methods instead of functions is for organization.
In addition to:
- providing method syntax
- and not having to repeat the type of `self` in every methods signature.

## Getters

``` rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}
```

Often use a field within a method of the same name for the purpose of getters. 

Rust does not implement getters automatically for struct fields.

Getters are useful because you can make the field private but the method public and enable read-only access to the field as part of an API.

### Where's the `->` Operators?

Rust does not have these operators due to **automatic referencing and dereferencing**.

Referencing and dereferencing is used in calling methods. It automatically adds in `&` `&mut` or `&` so object matches the signature of the method.

``` rust
p1.distance(&p2);
(&p1).distance(&p2);
```

Dereferencing and Automatic Referencing allows for both statements above to be equivalent.

### Methods with More Parameters







