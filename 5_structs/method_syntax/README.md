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







