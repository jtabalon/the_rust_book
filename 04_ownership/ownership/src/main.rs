fn main() {
    // s is not valid here, it's not yet declared
    // let s = "hello";    s is valid from this point forward

    // do stuff with s

    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`


} //scope is now over, and s is no longer valid
