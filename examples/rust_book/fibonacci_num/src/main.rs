use std::io;

fn main() {
    
    println!("Enter the nth fibonacci number!");

    let mut nth = String::new();

    io::stdin() 
        .read_line(&mut nth)
        .expect("Failed to read!");

    let nth: u32 = match nth.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Enter a number...");
            return;
        }
    };

    println!("The {nth}th Fibonacci number is...");

    if nth == 0 {
        println!("0 ... pick a better number");
        return;
    } else if nth == 1 {
        println!("0");
        return;
    }

    // initialise out totals and totals minus one and a place holder
    let mut tot: u64 = 1;
    let mut tot_1: u64 = 0;

    for _ in 2..=nth {

        // hold the temp
        let tot_holder = tot;

        // shift the totals
        tot = tot + tot_1;
        tot_1 = tot_holder;
    }
    println!("Fibbonacci number is: {tot}");
}
