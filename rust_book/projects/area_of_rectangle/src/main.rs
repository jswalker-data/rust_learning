
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

// refactor: define methods upon the Rectangle struct itself
// use implementation block to define the function within
// context of Rectangle
// use method syntax: instance.method(arguments) to call this function upon
// the instance
// still use &self as we are borrowing the instance, we are only reading and
// not writing
// can define a method same as one of the fields
// the method is rect1.width() whereas the field is rect1.width

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

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {rect1:#?}");

    if rect1.width() {
        println!("The rectangle has a nonzero width of {}", rect1.width);
    }

    println!(
        "The area of the rectangle is {} pixels!",
        rect1.area()
    );
}
