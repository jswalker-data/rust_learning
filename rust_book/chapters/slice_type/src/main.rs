// write a function that takes a string of words seperated by spaces
// and returns first word. Or whole string if no spaces

// without slices first

// take typs &String, don't need wonership
// return index of end of string (the space)
// we can;t return part of a string type
fn first_word(s: &String) -> usize {

    // convert string to array of bites
    let bytes = s.as_bytes();

    // create an iterator over the array of bites
    // iterate through the enumerated values to identify space
    // and retrn the index
    // &item needed as we are referencing an element
    for (i, &item) in bytes.iter().enumerate() {

        // search using the byte literal syntax
        if item == b' ' {
            return i;
        }
    }

    // if no space found, return entire string (so last index)
    s.len()
}



fn main() {
    let mut s = String::from("hello world");

    // word will return the value 5
    let word = first_word(&s);

    // empty s to make it equal to ""
    s.clear;

    // we are now having issues because word isn't inherintly 
    // connected to the s string, they are seperate and not actually
    // related so we could mut either with no issues
}

// use string slices

fn example_slices() {

    let s = String::from("hello world");

    // these are references to a portion of the string
    // [starting_index..ending_index + 1]
    // index 5 is not included in either of these slices!
    let hello = &s[0..5];
    let world = &s[6..11];

    // .. is range syntax
    let slice_1 = &s[0..2];
    let slice_2 = &s[..2];
    slice_1 == slice_2;

    // likewise for trailing numbers
    let l = s.len();
    let slice_3 = &s[3..l];
    let slice_4 = &s[3..];
    slice_3 == slice_4;

    // likewise
    let slice_5 = &s[0..l];
    let slice_6 = &s[..];

    // can only slice at a valid UTF-8 character boundary
    // slicing in middle of a multibyte character is an error
}

// string sliceing is '&str'
// if we have a string slice we can pass that and if we have
// &String we can slice the whole thing and take that as well
fn first_word_slices(s: &str) -> &str {

    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if &item = b' ' {
            return &s[0..i];
        }
    }

    // we output a string slice not a literal so need to slice
    // it all
    &s[..]

}

fn main_slices() {
    let mut s = String::from("hello world");

    // word will return the value 5
    let word = first_word_slices(&s);

    // error here!!!!
    s.clear;

    println!("the first wors is {word}");

    // error because we are immutable borowing s for the first_word_slice API
    // but then muttably destroying it
    // then calling the immutable borrow again later
}

// String literals
// chaging signature of first_word_slices means we can pass much more
fn main_3() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole.
    let word = first_word_slices(&my_string[0..6]);
    let word = first_word_slices(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s.
    let word = first_word_slices(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or
    // whole.
    let word = first_word_slices(&my_string_literal[0..6]);
    let word = first_word_slices(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word_slices(my_string_literal);
}

// while string slices are specific to strings, there is a more
// general slice type
// has type &[i32]
fn other_slices() {

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3]
    assert_eq!(slice, &[2, 3])
}