struct Song {
    is_released: bool,
    name: String,
    duration_in_seconds: u16,
}

fn build_song(is_released: bool, name: String, duration_in_seconds: u16) -> Song {
    Song {
        is_released,
        name,
        duration_in_seconds,
    }
}

fn main() {
    let my_song = Song {
        is_released: true,
        name: String::from("The Very Thought Of You"),
        duration_in_seconds: 228,
    };

    println!(
        "name: {}, is released: {}, duration: {}s",
        my_song.name, my_song.is_released, my_song.duration_in_seconds
    );

    let mut my_mutable_song = Song {
        is_released: true,
        name: String::from("The Very Thought Of You"),
        duration_in_seconds: 228,
    };

    my_mutable_song.is_released = true;

    println!("{}", my_song.is_released); // true

    let my_quick_song = build_song(false, String::from("speedy"), 12);
    println!("{}", my_quick_song.name); // speedy

    let my_derivative_song = Song {
        duration_in_seconds: 114,
        ..my_song
    };
    // println!("{}", my_song.name) invalid because it has been moved.
    println!("{}", my_derivative_song.name); // The Very Thought of You

    println!("{}", my_song.duration_in_seconds); // This works because it is a copied type and so
    // it copied and not moved
}
