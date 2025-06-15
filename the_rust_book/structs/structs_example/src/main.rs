// By default structs do not implement the ability to print in their entirity
#[derive(Debug)] // So we have to opt in
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn set_width(&mut self, width: u32) {
        self.width = width
    }
    fn set_height(&mut self, height: u32) {
        self.height = height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        (self.width > other.width && self.height > other.height)
            || (self.height > other.width && self.width > other.height)
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

    rect.set_width(55);
    rect.set_height(33);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    let rect1 = Rectangle {
        width: 20,
        height: 40,
    };

    let rect2 = Rectangle {
        width: 40,
        height: 20,
    };

    println!(
        "Rect {} fit rect1 in it",
        if rect.can_hold(&rect1) {
            "can"
        } else {
            "cannot"
        }
    );
    println!(
        "Rect {} fit rect2 in it",
        if rect.can_hold(&rect2) {
            "can"
        } else {
            "cannot"
        }
    );

    println!("Created a square with size {}", Rectangle::square(24).width)
}
