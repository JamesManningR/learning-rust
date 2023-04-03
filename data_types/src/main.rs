fn main() {
    // Scalar types ========================================
    // Integers
    let intDefault = 5; // Signed 32 bit integer (rust defaults to i32 if no type is specified)

    let int8: i8 = 5; // Signed 8 bit integer
    let intUnsigned8: u8 = 5; // Unsigned 8 bit integer

    let int32: i32 = 5; // Signed 32 bit integer
    let intUnsigned32: u32 = 5; // Unsigned 32 bit integer (32 bit intager)

    let int64: i64 = 5; // Signed 64 bit integer
    let intUnsigned64: u64 = 5; // Unsigned 64 bit integer

    let int128: i128 = 5; // Signed 128 bit integer
    let intUnsigned128: u128 = 5; // Unsigned 128 bit integer

    let intArch: isize = 5; // Signed integer, the size of the pointer
    let intUnsignedArch: usize = 5; // Unsigned integer, the size of the pointer

    // Floating point
    let floatDefault = 5.0; // 64 bit floating point number (rust defaults to f64 if no type is specified)

    let float32: f32 = 5.0; // 32 bit floating point number
    let float64: f64 = 5.0; // 64 bit floating point number

    // Boolean
    let boolTrue = true;
    let boolFalse = false;

    // Characters (supports unicode)
    let char = 'a'; // Single character
    let charEmoji = '😻'; // Single character

    // Compound types ========================================
    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1); // Tuples can contain different types
    let (x, y, z) = tup; // Destructuring a tuple
    println!("The value of y is: {y}"); // 6.4

    let fiveHundred = tup.0; // Accessing a tuple element
    let sixPointFour = tup.1; // Accessing a tuple element
    let one = tup.2; // Accessing a tuple element

    // Arrays
    let arr = [1, 2, 3, 4, 5]; // Arrays are fixed length
    let first = arr[0]; // Accessing an array element
    let second = arr[1]; // Accessing an array element
}
