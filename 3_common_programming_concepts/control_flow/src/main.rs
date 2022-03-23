fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // This doesn't work:
    // let number = 3;
    // if number {
    //     println!("number was three");

    if number != 0 {
        println!("number was something other than zero");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // In Rust, you can assign variables to if statements, as follows:

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

    // Note: The following throws an error due to mismatched types:
    // In other words, types must match when assigning variables to an if statement

    // let condition = true;
    // let number = if condition { 5 } else { "six" };
    // println!("The value of number is: {}", number);

    // Different types of Loops:

    // 1. loop

    // Executes a block of code until you tell it to stop (CTRL+C).

    // loop {
    //     println!("Again!")
    // }

    // Keywords: break + continue
    // These apply to the innermost loop at the point
    // break: tell probram to stop executing within code
    // continue: skip over remaining code in the loop and go to next iteration
    
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);

    // Common use case: use    

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    // 2. while loops

    let mut num = 3;

    while num != 0 {
        println!("{}!", num);

        num -= 1;
    }

    println!("LIFTOFF!!!");

    // 3. for loops

    let a = [10, 20, 30, 40, 50];

    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    for element in a {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() { // .rev() reverses the order!!
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");















    
    
    

}
