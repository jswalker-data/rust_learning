
// this is what ownership_4 would look like with a reference
// rather then transfering the pointer

fn main() {
    
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.")
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
