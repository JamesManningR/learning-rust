# Strucs

A struct is a custom data type. Structs are similar to tuples in that they hold
multiple related values. Unlike tuples, you name each piece of data so it's clear
what each value is.

## How do I define a struct?

Using the `struct` keyword, you can provide the key names and types to define the
data structure.

```rust
struct Song {
    is_released: bool,
    name: String,
    duration_in_seconds: u16,
}
```

As you can see, we now have a structure which contains related data about a song.

We can then use the struct to create an `instance` of that struct.

```rust
fn main() {
    let my_song = Song {
        is_released: true,
        name: String::from("The Very Thought Of You"),
        duration_in_seconds: 228,
    };
}
```

## How do I use the struct

Like most C-style languages, we use the `.` character to access the individual
values inside of a struct.

```rust
println!(
    "name: {}, is released: {}, duration: {}s",
    my_song.name, my_song.is_released, my_song.duration_in_seconds
)
```

reassignemnt works as you might expect (if the instance is immutable of course)

```rust
let mut my_mutable_song = Song {
    is_released: true,
    name: String::from("The Very Thought Of You"),
    duration_in_seconds: 228,
};

my_mutable_song.is_released = true;

println!("{}", my_song.is_released); // true
```

Partial immutability isn't a thing in rust, unless you use a `Cell` or `RefCell`
So a structs mutability is all-or-nothing

## Syntactic Sugar

### Shorthand

Structs can be constructed using shorthand notation.

```rust
fn build_song(is_released: bool, name: String, duration_in_seconds: u16) -> Song {
    Song {
        is_released,        // Since the names and types are the same
        name,               // We can just pass the names without the :
        duration_in_seconds // to save redundant repetition
    }
}
```

### Struct update syntax

Structs can also be initialised using a spread shorthand

```rust
let my_derivative_song = Song {
    name: String::from("The Very Thought of Her"),
    ..my_song // Use remaining fields from my_song
};
```

Note that non-scalar types are moved rather than copied, meaning the original struct
no longer owns them.

Take our name for example. If we spread the original `my_song` struct into our new
song struct, without adding a new name, we will not be able to access the original
name via the `my_song struct`.

```rust
let song1 = Song {
    name: String::from("Hello"),
    artist: String::from("Adele"),
    duration: 180,
};

let song2 = Song {
    name: String::from("Goodbye"),
    ..song1
};

println!("{}", song1.artist) // ❌ error - song1.artist was moved
println!("{}", song1.name) // ✅ "Hello"
println!("{}", song2.artist) // ✅ "Adele"
println!("{}", song2.name) // ✅ "Goodbye"
```

```rust
let song2 = Song {
    name: String::from("Goodbye"),
    artist: song1.artist.clone(),  // Explicit clone
    duration: song1.duration,
};
// song1 is still fully usable
```

```rust
struct SongRef<'a> {
    name: &'a str,
    artist: &'a str,
    duration: u32,
}
```

## Tuples Structs

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let black2 = origin
}
```

These typles are the same stucture but they are different types
A function taking in a Color would have to be a Color and cannot be a type of
point.

### Tuple Structs vs Tuples

Tuple structs are named types that behave like tuples.
**Key differences**

- Destructuring
    a. Tuples use let (x, y) = tuple,
    b. tuple structs require the type name let Point(x, y) = point
- Type identity - Point(1, 2) and (1, 2) are different types
- Methods- Tuple structs can have custom methods and trait implementations

## No Field Structs

Structs can be created without any values

```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```

Why will be revealed later when learning about *traits*
