// A slice lets you reference a contiguous sequence of elements
// in a collection, rather than the whole collection

// A slice is a reference so it does not have ownership

fn main() {
    let s = String::from("Hello world");
    // Here's how we create a slice
    // A slice references the s variable
    let hello = &s[0..5]; // Remember, you can't mutate s now, as it's immutable
    let _world = &s[6..11];
    println!("{hello}");

    println!("{hello}");

    let word = get_first_word_length(&s);
    let first_word = get_first_word(&s);

    println!("The first word of {s} is {word} characters long");
    println!("The first word of {s} {first_word}")
}

// Finds the first word in a string besed on the first
// occurance of a space character
// If there isn't a space, then it should return the whole string
// For now, we'll return the index
fn get_first_word_length(string: &String) -> usize {
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

fn get_first_word(string: &String) -> &str {
    let length = get_first_word_length(string);

    &string[0..length]
}
