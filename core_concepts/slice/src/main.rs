// A slice lets you reference a contiguous sequence of elements
// In a collection, rather than the whole collection

// A slice is a reference so it does not have ownership

fn main() {
    println!("Hello, world!");
}

// Finds the first word in a string besed on the first
// occurance of a space character
// If there isn't a space, then it should return the whole string
// For now, we'll return the index
fn first_word(string: &String) -> usize {

}