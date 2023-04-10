// A slice lets you reference a contiguous sequence of elements
// In a collection, rather than the whole collection

// A slice is a reference so it does not have ownership

fn main() {
    let s = String::from("Hello world");

    let word = first_word(&s);

    println!("The first word of {s} is {word} characters long");
}

// Finds the first word in a string besed on the first
// occurance of a space character
// If there isn't a space, then it should return the whole string
// For now, we'll return the index
fn first_word(string: &String) -> usize {
    // First turn the string into an array of the characters as byte representation
    let bytes = string.as_bytes();

    // Loop through the bytes in the array
    // this is a little complex right now, but here is a summary of how this loop works

    // iter returns each element in a collection,
    // and enumerate turns that into a tuple
    // the first element in the tuple is the index, and the second element is a reference to the value
    // this is why we're able to destructure them using the (i, &item) syntax
    for (i, &item) in bytes.iter().enumerate() {

        // b' ' referes to the byte representation of the character (in this case a space character)
        // This is called a byte literal
        if item == b' ' {
            // If we find it, return the current index
            return i;
        }
    }

    // Otherwise we haven't found one, so we'll return the last index (the length of the string)
    string.len()
}