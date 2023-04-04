fn main() {
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
