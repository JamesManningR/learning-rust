fn main() {
    let number = 3;

    // Expressions must evaluare to a bool. No implicit truthy or falsy values
    if number < 5  {
        println!("Number is less than 5");
    } else {
        println!("Number is more than 5");
    }
    
    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 2, 3 or 4");
    }

    // You can use if in a let statement
    let is_even = if number % 2 == 0 { true } else { false };
    println!("is_even: {}", is_even)
}
