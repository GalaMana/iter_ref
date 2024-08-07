use iter_ref::prelude::*;

fn main() {
    for letter in vec!['W', 'o', 'r', 'k', 's', '?'].iter_better() {
        print!("{}", letter.to_string())
    }

    println!();

    for letter in (&['W', 'o', 'r', 'k', 's', '?']).iter_better() {
        print!("{}", letter.to_string())
    }

    println!();

    let slice = &mut ['W', 'o', 'r', 'k', 's', '?'];
    for letter in slice.iter_mut() {
        letter.make_ascii_uppercase()
    }

    for letter in slice.iter_better_mut() {
        print!("{}", letter.to_string())
    }

    println!();
}