const GIFTS: [&str; 12] = [
    "partridge in a pear tree.",
    "turtle doves",
    "French hens",
    "calling birds",
    "golden rings",
    "geese a-laying",
    "swans a-swimming",
    "maids a-milking",
    "ladies dancing",
    "lords a-leaping",
    "pipers piping",
    "drummers drumming",
];

fn get_ordinal(num: usize) -> String {
    let r = num % 10;
    match r {
        1 => format!("{}st", num),
        2 => format!("{}nd", num),
        3 => format!("{}rd", num),
        _ => format!("{}th", num),
    }
}

fn main() {
    for (i, el) in GIFTS.iter().enumerate() {
        let ordinal = get_ordinal(i + 1);
        println!("On the {ordinal} day of Christmas");
        println!("My true love gave to me:");
        
        if i == 0 {
            println!("a {el}");
        } else {
            for amount in 1..=i+1 {
                let gift = GIFTS[amount - 1];
                
                println!("{amount} {gift}");
            }
        }   
        println!();
    }
}

// Lyrics (summary): 
// On the nth day of Christmas
// My true love gave to me
// Twelve drummers drumming,
// Eleven pipers piping,
// Ten lords a-leaping,
// Nine ladies dancing,
// Eight maids a-milking,
// Seven swans a-swimming,
// Six geese a-laying,
// Five golden rings,
// Four calling birds,
// Three French hens,
// Two turtle doves
// And a partridge in a pear tree.