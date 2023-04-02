fn main() {
    // Setting up x without the mut keyword
    let x = 5;
    println!("The value of x is: {x}");

    // Re-assigning will produce an error
    x = 6;
    println!("The value of x is: {x}");
}
