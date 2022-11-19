// Rust is a statically typed language.
//   - It must know types of all variables at compile time.
//   - Signed versus unsigned: signed allows for negative values
//   - Rust integers default to i32.
//   Two complement wrapping if overflow:
//   - 


fn main() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    // floating point numbers default to f64 (f64 just as fast as f32, and more precise)
    println!("x = {}, y = {}", x, y);

    // Common Operations

    // addition
    let sum = 5 + 10;
    println!("sum = {}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("difference = {}", difference);

    // multiplication
    let product = 4 * 30;
    println!("product = {}", product);

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0
    println!("quotient = {}", quotient);
    println!("floored = {}", floored);

    // remainder
    let remainder = 43 % 5;
    println!("remainder = {}", remainder);

    // Boolean Types

    let t = true;
    let f: bool = false; // with explicit type annotation
    println!("t = {} and f = {}", t, f);

    // Character Type

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("c = {} and z = {} but heart_eyed_cat = {}", c, z, heart_eyed_cat);
    // NOTE: char literals are specified with SINGLE quotes
    // char is 4 bytes in size


    // Compound Types: Tuples + Arrays
    // We'll work with tuples first.
    // Types within a tuple may vary.
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    // Tuple indexing with a period.
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("The value of five_hundred is: {} and the value of six_point_four is {} and the value of one is: {}", five_hundred, six_point_four, one);

    // Time to work with arrays.

    let a = [1, 2, 3, 4, 5];

    // Arrays are useful when you want data allocated on the stack versus the heap.
    // Vectors are flexible in size (dynamic data type)

    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];
    // will always stay finite as 12 months, so use an array.

    let b: [i32; 5] = [1, 2, 3, 4, 5];
    // types of elements.

    let c = [3; 5];
    // this is the same as c = [3, 3, 3, 3, 3]

    // TODO: troubleshoot printing arrays.

    // so i don't know why we can't do this lol
    // println!("{}", months);
    // println!("a = {}, b = {}", a, b);

    // Array indexing
    let first = a[0];
    let second = a[1];
    println!("first = {}, second = {}", first, second);

    // Now refer to invalid array indexing one directory above.
}

