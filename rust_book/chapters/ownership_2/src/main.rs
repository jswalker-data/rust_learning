fn main() {
    
    // s comes into scope
    let s = String::from("hello");
    
    // s's value moves into the function and is no longer valid here
    takes_ownership(s);

    // x comes into scope
    let x = 5;

    // because i32 implements the Copy trait, x does NOT move into
    // the function, so it's okay to use x afterwards
    makes_copy(x);

    // at this point s is gone and x still exists

// here, x goes out of scope, then s
// however s value was moved so nothing happens
}

fn takes_ownership(some_string: String) {
    
    // some_string comes into scope
    println!("{some_string}");

// some_string goes out of scope and 'drop' called
// backing memory is freed
}

fn makes_copy(some_integer: i32) {

    // some_integer comes into scope
    println!("{some_integer}");

// some_integer goes out of scope, nothing happens
}




