fn main() {
    // Adding mut now will allow the variable to be re-assigned a value
    let mut x = 5;
    println!("The value of x is: {x}");

    // This will no longer produce an error
    x = 6;
    println!("The value of x is: {x}");
}
