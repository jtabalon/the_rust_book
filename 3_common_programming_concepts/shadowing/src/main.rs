fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
        // The value of x in the inner scope is: 12
    }
    println!("The value of x is: {}", x);
    // The value of x is: 6

    // New section of book here. 

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is {}", spaces);

    let mut spaces = "   ";
    spaces = spaces.len();
    // Unable to mutate a variable's type
    println!("The value of spaces is {}, because we cannot mutate a variable's type.", spaces);

}
