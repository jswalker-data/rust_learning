fn main() {

    // floating-point types
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; //Result is -1

    // remainder
    let remainder = 43 % 5;

    // booleans
    let t = true;

    let f: bool = false;

    // character type - literals with single quotes
    let c = 'z';
    let x: char = 'Z';
    let heart_eyed_cat = '😻';

    // tuple types
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // pattern matching - deconstruction
    let (x, y, z) = tup;
    //println!("The value of y is: {y}") // 6.4

    // direct access
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    // arrays
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5]; // [3, 3, 3, 3, 3]

    let first = a[0];
    let second = a[1];

}