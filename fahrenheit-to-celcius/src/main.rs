// C = 5/9(F-32).
// F = 9/5C + 32.

const CEL_TO_FAH_RATIO: f64 = 9.0 / 5.0;
fn celcius_to_fahrenheit (celc: f64) -> f64 {
    (celc * CEL_TO_FAH_RATIO) + 32.0
}

const FAH_TO_CEL_RATIO: f64 = 5.0 / 9.0;
fn fahrenheit_to_celcius (fahr: f64) -> f64 {
    (fahr - 32.0) * FAH_TO_CEL_RATIO
}

fn main() {
    // Test cases
    // 40c = 
    let celc = celcius_to_fahrenheit(25.0); // Expected: 77
    println!("{celc}");
    let fahr = fahrenheit_to_celcius(25.0); // Expected: -3.888888888888889
    println!("{fahr}");
}
