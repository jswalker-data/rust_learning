
// Each value in rust has an owner
// There can only be one owner at a time
// When the owner goes out of scope, the value is dropped

// Scope, see what happens for items on the heal as the stack is trivial
fn main() {

    // String types, mutable. Compared to literals whch are not (hardcoded)
    let mut s= String::from("hello");
    
    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{s}"); // print 'hello, world!'



}