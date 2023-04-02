fn main() {
    // Adding a const, This cannot be computed at runtime as it is computed on compilation
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // Adding mut now will allow the variable to be re-assigned a value
    let mut x = 5;
    println!("The value of x is: {x}");

    // This will no longer produce an error
    x = 6;
    println!("The value of x is: {x}");
}
