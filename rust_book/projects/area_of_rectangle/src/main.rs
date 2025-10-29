
// using structs to calulate area of a rectangle
// start by using single variables then refactor
// to use structs
//commits will show refactors and comments below will show stages

// refactor: change dual input to a single input, we can't see that
// these are related in any ways at the moment

fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} pixels!",
        area(rect1)
    );
}


fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}