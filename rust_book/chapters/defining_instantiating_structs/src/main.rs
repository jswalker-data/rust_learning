
// similar to tuples, multiple related values
// similar: can be different types, diff: named in struct
// fields for entries

struct user {
    active: bool,
    username: String,
    email: String,
    sing_in_count: u64,
}

// create an instance of a struct
fn main_1() {
    let user1 = User {
        active: true,
        usernmae: String::from("someone123"),
        email: String::from("someone@example.com"),
        sing_in_count: 1,
    };

    // dot notation to access information 'user1.email'
    // if instance is mut can change value using dot notation
    // entire instance mut or not mut
    user1.email = String::from("anotheremail@example.com")

}

// as with any experssion, we can contruct a new instance of the struct
// as the last expression in a function to implicitly return the new instance
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

// tedious to assign username to username etc...
// field init shorthand
// email and username have same name as the field
fn build_user_init(email: String, username: String) -> User {
    User {
        active: true,
        username,
        emial,
        sign_in_count: 1,
    }
}
// struct update syntax
// can reference other instances
// ..x says update all other fields not explicitly defined here
fn main_2() {
    let user1 = User {
        active: true,
        usernmae: String::from("someone123"),
        email: String::from("someone@example.com"),
        sing_in_count: 1,
    };

    let user_2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count
    };

    let user_3 = User {
        emial: String::from("another_2@example.com"),
        ..user1
    };
}