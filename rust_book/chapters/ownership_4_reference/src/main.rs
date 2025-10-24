
// this is what ownership_4 would look like with a reference
// rather then transfering the pointer

fn main() {
    
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.")
}

// s is a reference to a String
fn calculate_length(s: &String) -> usize {
    s.len()

// Here, s goes out of scope. But because s does not have ownership of 
// what it refers to, the String is not dropped
}
