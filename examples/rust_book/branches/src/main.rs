fn main() {
    let num = 6;

    if num % 4 ==0 {
        println!("number is divisible by 4");
    } else if num % 3 == 0 {
        println!("number is divisible by 3");
    } else if num % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number not divisible by 4, 3, 2");
    }

    // in assignments outputs must be same type in each arm
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}")

}