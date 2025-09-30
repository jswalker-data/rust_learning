fn main() {
    println!("Hello, world!");
    
    another_function(5);

    print_labeled_measurement(5, 'h');

    // expressions evaluate to a value, statements do not
    // let x = (let = 6) does not run as it is a statement

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is {y}");

    let x = five();
    println!("The value of x is: {x}")
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// return values, implicitly return last value if no return statement
fn five() -> i32 {
    5
}