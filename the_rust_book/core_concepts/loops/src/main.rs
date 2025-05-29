fn main() {
    loops();
    while_loops();
    for_loops();
}

fn loops() {
    // rust has 3 types of loops
    // loop, while and 'for'

    // loop
    // Loop runs forever until it's explicitly exited
    let mut counter = 0;

    loop {
        println!("And another one {counter}");

        counter += 1;

        if counter >= 10 {
            break;
        };
    }

    // Loops can also return values using the break keyword
    counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("This should equal 20: {result}");

    // You can optionally label a loop using a ' prefix
    // so you can use the break or continue keyword for a specific loop
    // This is used for nested loops
    let mut count = 0;
    'count_up: loop {
        println!("count: {count}");
        let mut remaining = 10;

        loop {
            println!("remaining: {remaining}");
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'count_up;
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("end count: {count}");
}

fn while_loops() {
    // While loops are essentially loops with condition checks at the start of a loop
    // This means that if the condition is met
    let mut number = 3;

    while number != 0 {
        println!("before: {number}");

        number -= 1;

        println!("after: {number}");
    }

    // number: 0
    number = 5;

    while number != 1 {
        println!("before: {number}");

        number -= 2;
        // Notice that it doesn't matter if the condition is true mid way through the function
        println!("mid: {number}");
        number += 1;

        println!("after: {number}");
    }
}

// We can use for loops to loop through a collection
fn for_loops() {
    let arr = [10, 20, 30, 40, 50];

    for el in arr {
        println!("The value is: {el}");
    }

    // We can also loop over a range using the range util
    // Note that this is not inclusive
    for num in 1..4 {
        println!("{num}")
    }

    // and in reverse
    for num in (1..4).rev() {
        println!("{num}")
    }
}

