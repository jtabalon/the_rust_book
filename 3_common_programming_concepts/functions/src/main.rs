// 1. First introduction to functions
// fn main() {
//     println!("Hello, world!");

//     another_function();
// }

// fn another_function() {
//     println!("Another function.");
// }


// 2. Introduction to Parameters

// fn main() {
//     another_function(5);
// }

// fn another_function(x: i32) {
//     println!("The value of x is: {}", x);
// }

// You must specify parameter types in the function definition.
// This was a design choice by Rust designers.


// 3. Multiple Parameters

// fn main() {
//     print_labeled_measurement(5, 'h');
// }

// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {}{}", value, unit_label);
// }


// 4. Statements and Expressions

// Statements perform an action and do NOT return a value.
// Expressions evaluate to a resulting value.

fn main() {
    let y = {
        let x = 3;
        x + 1 // no semicolon because it's an expression.
              // expressions don't include ending semicolons.
    };

    println!("The value of y is: {}", y);
}






