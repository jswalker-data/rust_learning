
// using structs to calulate area of a rectangle
// start by using single variables then refactor
// to use structs
//commits will show refactor

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} pixels!",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}