fn main() {
    // Adding a const, This cannot be computed at runtime as it is computed on compilation
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("3hr in secs: {THREE_HOURS_IN_SECONDS}");

    // Adding mut now will allow the variable to be re-assigned a value
    let mut x = 5;
    println!("The value of x is: {x}");

    // This will no longer produce an error
    x = 6;
    println!("The value of x is: {x}");

    let x = x + 6;
    println!("The value of x is: {x}");
    
    // Example of shadowing
    let x = "Pringles";
    println!("The value of x is: {x}");

    // Example of shadowing
    let x = 12;
    println!("The value of x is: {x}");
    
    {
        let x = "Shadowing in a scope?";
        println!("The value of x in this scope is: {x}");
    }

    println!("Oh look, now back in this scope: {x}");

}
