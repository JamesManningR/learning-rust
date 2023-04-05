fn main() {
    let s = String::from("Hello");

    print_my_string(s);
    
    // println!("{s}");
    // This would cause an error since the print_my_string(s) function
    // takes ownership of the s variable.
    // we can get around this by transferring ownership back by assigning it to another variable
    let s2 = String::from("Hi");
    let s3 = print_my_string_and_return(s2);

    let x = 5;

    print_my_int(x);
}

fn print_my_string(a_string: String) {
    println!("{a_string}");
}

fn print_my_string_and_return(a_string: String) -> String {
    println!("{a_string}");

    a_string
}

fn print_my_int(an_int: i32) {
    println!("{an_int}");
}