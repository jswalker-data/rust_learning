
// using structs to calulate area of a rectangle
// start by using single variables then refactor
// to use structs
//commits will show refactors and comments below will show stages

// refactor: change dual input to a single input, we can't see that
// these are related in any ways at the moment

// refactor: the tuple doesn't give us any info on what is height and
/// what is width. let's change to a struct for more information

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} pixels!",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}