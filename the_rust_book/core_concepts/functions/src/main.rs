fn out_of_main() {
    println!("Hello, world!");
}

fn main() {
    // Note that functions can go before
    out_of_main();
    // Or after the current funciton, as long as it's in the scope
    with_param(11);
    with_param(5);
    with_multiple_params(21, 'm');
    expression(3); // Notice that this really does nothing of use
    let two_squared = expression(2); // It needs to be assigned/used
    println!("{}", two_squared);
}


// Function params must be typed
fn with_param(x: i32) {
    println!("The value of x is {x}");
}

// Function params must be typed
fn with_multiple_params(value: i32, label: char) {
    println!("{label}: {value}");
}

// An expression returns something.
// Otherwise it's a procedure
fn expression(num: i32) -> i32 {
    num * num
}