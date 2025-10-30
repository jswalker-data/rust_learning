
// using structs to calculate area of a rectangle
// start by using single variables then refactor
// to use structs
//commits will show refactors and comments below will show stages

// refactor: change dual input to a single input, we can't see that
// these are related in any ways at the moment

// refactor: the tuple doesn't give us any info on what is height and
// what is width. let's change to a struct for more information

// addition: I want to see rect1 in an output form for debugging
// println! does not know how to format it so errors
// we can print using {:?} which is debug mode (:#? is pretty debug)
// however we need to opt into it by adding the outer attribute
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

// addition: new method to see if rectangles can fit inside one another
// remove the width method for simplicity and the printing

// addition: associated functions don't have self, as not methods, often used
// for constructors. retuning new instance of struct
// self in return and body are aliases for Rectangle type. We are
// returning a Rectangle type and defining a Rectangle struct here

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        let w = self.width >= rect.width;
        let h = self.height >= rect.height;
        w && h
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let _sqr = Rectangle::square(3);

    println!("Can rect1 hold rect 2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect 3? {}", rect1.can_hold(&rect3));

}
