fn main() {
    // Scalar types ========================================
    // Integers
    let int_default = 5; // Signed 32 bit integer (rust defaults to i32 if no type is specified)

    let int8: i8 = 5; // Signed 8 bit integer
    let int_unsigned8: u8 = 5; // Unsigned 8 bit integer

    let int32: i32 = 5; // Signed 32 bit integer
    let int_unsigned32: u32 = 5; // Unsigned 32 bit integer (32 bit intager)

    let int64: i64 = 5; // Signed 64 bit integer
    let int_unsigned64: u64 = 5; // Unsigned 64 bit integer

    let int128: i128 = 5; // Signed 128 bit integer
    let int_unsigned128: u128 = 5; // Unsigned 128 bit integer

    let int_arch: isize = 5; // Signed integer, the size of the pointer
    let int_unsigned_arch: usize = 5; // Unsigned integer, the size of the pointer

    // Floating point
    let float_default = 5.0; // 64 bit floating point number (rust defaults to f64 if no type is specified)

    let float32: f32 = 5.0; // 32 bit floating point number
    let float64: f64 = 5.0; // 64 bit floating point number

    // Boolean
    let bool_true = true;
    let bool_false = false;

    // Characters (supports unicode)
    let char = 'a'; // Single character
    let char_emoji = '😻'; // Single character

    // Compound types ========================================
    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1); // Tuples can contain different types
    let (x, y, z) = tup; // Destructuring a tuple
    println!("The value of y is: {y}"); // 6.4

    let five_hundred = tup.0; // Accessing a tuple element
    let six_point_four = tup.1; // Accessing a tuple element
    let one = tup.2; // Accessing a tuple element

    // Arrays
    let arr = [1, 2, 3, 4, 5]; // Arrays are fixed length
    let first = arr[0]; // Accessing an array element
    let second = arr[1]; // Accessing an array element
    let typed_arr: [i32; 5] = [1, 2, 3, 4, 5]; // Arrays can be typed
    let all_zeros = [0; 5]; // Arrays can be initialized with a value
}
