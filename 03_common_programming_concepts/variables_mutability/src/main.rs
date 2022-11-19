fn main() {
    // constants are ALWAYS immutable by default.
    // constants are valid within the scope they were declared in.

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // let x = 5;
    // println!("The value of x is : {}", x);
    // x = 6;
    // println!("The value of x is : {}", x);
    // ^^^^^ cannot assign twice to immutable variable
    let mut x = 5;
    println!("The value of x is : {}", x);
    x = 6;
    println!("The value of x is : {}", x);
    // successfully compiles
}
