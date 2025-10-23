
// Each value in rust has an owner
// There can only be one owner at a time
// When the owner goes out of scope, the value is dropped

// Scope, see what happens for items on the heal as the stack is trivial
fn main() {

    // String types, mutable. Compared to literals whch are not (hardcoded)
    let mut s= String::from("hello");
    
    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{s}"); // print 'hello, world!'

    // data interactions, both onto the stack and are clones equal to 5
    // no need to clone as fixed, known size and is arbitrary to always clone it
    let x = 5;
    let y = x;

    // string version
    // copy the pointer, kength and capacity on the stack. Not the data on the heap
    let s1 = String::from("hello");
    let s2 = s1;

    // if we free here then double free error would occur as both go out of scope
    // at the same time
    // s1 moved to s2, s1 is not valid anymore, it is moved to s2

    // assigning comletely new variable
    // nothing is referring to the original on the heap
    // nothing refers to 'hello' on the heap
    let mut s = String::from("hello");
    s = String::from("ahoy");

    println!("{s}, world!");

    // deep copy by using clone, duplicates the heap data
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");

    // copy can only be on stack items and not heap allocations

}