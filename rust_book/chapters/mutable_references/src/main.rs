
//how we can mut a reference
// can only borrow one mutable reference at a time
// and can take neither a mutable or immutable reeference either
// reference has to go out of scope before we can create a new mutable or immutable

fn main() {

    // make s mutable
    let mut s = String::from("hello");

    // then create a mutable reference
    change(&mut s);

}

// update function signature to accept mutable references
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
