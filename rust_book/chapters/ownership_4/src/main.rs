
// use a value but not own it

fn main() {

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{s2}' is {len}.");

}
// we have to return it to the main function in order to use s again
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)

}