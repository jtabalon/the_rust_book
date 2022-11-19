// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width1, height1)
//     )
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// Refactoring with Tuples for consiseness

// fn main() {
//     let rect1 = (30, 50);
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     )
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        // width: 30,
        width: dbg!(30 * scale),
        height: 50,
    };

    println!("rect1 is {:#?}", rect1); // Note {:#?} for debug/printing struct

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    dbg!(&rect1); // dbg macro takes ownership of expression, and prints where it gets called, and returns ownership fo value // dbg macro takes ownership of expression, and prints where it gets called, and returns ownership fo value.
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
















