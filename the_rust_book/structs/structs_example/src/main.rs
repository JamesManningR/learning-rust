// By default structs do not implement the ability to print in their entirity
#[derive(Debug)] // So we have to opt in
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn set_width(&mut self, width: u32) {
        self.width = width
    }
    fn set_height(&mut self, height: u32) {
        self.height = height
    }
}

fn main() {
    let mut rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("Debug Rectangle: {rect:#?}"); // :? prints the strucy # makes it pretty

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    rect.set_width(5);
    rect.set_height(3);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );
}
