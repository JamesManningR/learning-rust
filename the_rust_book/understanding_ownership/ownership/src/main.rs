fn main() {
    let s = String::from("Hello");

    print_my_string(s);

    // println!("{s}");
    // This would cause an error since the print_my_string(s) function
    // takes ownership of the s variable.
    // we can get around this by transferring ownership back by assigning it to another variable
    let s2 = String::from("Hi");
    let _s3 = print_my_string_and_return(s2);

    // We can also instead use references rather than the values themselves
    let str_for_ref = String::from("Memes");
    print_my_string_ref(&str_for_ref);
    // As you can see, we can use it again without any issues
    println!("{str_for_ref}");

    let x = 5;

    print_my_int(x);

    let mut mutable_string = String::from("Rust memes are hard to find");

    // If we want to use mutable references, then we need to explicitly define this behaviour
    // we do this by providing the &mut to the argumnet as well as in the function
    mutate_my_string(&mut mutable_string);
    println!("{mutable_string}");
}

fn print_my_string(a_string: String) {
    println!("{a_string}");
}

fn print_my_string_and_return(a_string: String) -> String {
    println!("{a_string}");

    a_string
}

fn print_my_string_ref(a_string: &String) {
    println!("{a_string}");
    // Note we cannot mutate this in its current state
    // This would throw a error
    // a_string.push_str(" lmao");
}

// Not this function can access and mutate the referenced variable
fn mutate_my_string(a_string: &mut String) {
    a_string.push_str(" because of the video game");
}

fn print_my_int(an_int: i32) {
    println!("{an_int}");
}
