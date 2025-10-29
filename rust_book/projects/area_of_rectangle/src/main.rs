
// using structs to calulate area of a rectangle
// start by using single variables then refactor
// to use structs
//commits will show refactors and comments below will show stages

// refactor: change dual input to a single input, we can't see that
// these are related in any ways at the moment

// refactor: the tuple doesn't give us any info on what is height and
// what is width. let's change to a struct for more information

// addition: I want to see rect1 in an output form for debugging
// println! does not know how to format it so errors
// we can printusing {:?} which is debug mode (:#? is pretty debug)
// however we need to opt into it by adding the outer attriute
// just before struct definition. This is attached to the struct
// could also use dbg!() which borrows and returns the values for debugging
// rather then taking ownership

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {rect1:#?}");

    println!(
        "The area of the rectangle is {} pixels!",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}