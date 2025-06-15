# Enums

Enums (Enumerations) are a way to define a finite list of possible variants.
This allows the compiler to use a minimum representation of the variant, while still
keeping the developer experience

Enums are interesting in rust. They not only represent an enumeration of a value like in
typescript. They can also hold structures in themselves.

```rust
enum IpAddrKind {
    V6,
    V4,
}
```

So here you can see the kind you might recognise.

but you can also enumerate types

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

Here you can see the `Message` can be:

- A `Quit` message, which has no data.
- A `Move` message, which has an `x` and `y` coordinate
- A `Write` message, which has a `String`
- A `ChangeColor`, which takes 3 values (for r g and b)

These can then be used so create these message types

```rust
let my_quit_message = Message::Quit();
let my_move_message = Message::Move({x: 12, y: 24});
let my_write_message = Message::Write(String::from("Hello World"));
let my_change_color_message = Message::ChangeColor(120,20,203);
```

## Null / Optional values

In rust there is no concept of `null` instead rust provides an enum `Option<T>`
Using the Generic, this can represent an optional value which may be there or
not.

This is the only example of any equivilent of null within the language.
Doing it this way has a safety advantage. it means when working with these kinds
of values, you must handle both cases, if it's there or if it's not there.

This is how it's defined in the standard library

```rust
enum Option<T> {
    None,
    Some(T),
}
```

This implementation does something unexpected. It creates the issue of using a
possibly undefined value with a definitely defined one, not by having a special
case, but instead by just not having an implementation for `T` interacting with
`Option<T>` which I think is pretty neat. It's not that it is specifically
warning you against creating null errors, it's just that the compiler doesn't know
how two different types can work with each other.

The only way to then get `T` to interact with `Option<T>` is by handling the case
where `T` is nullish. Forcing you to handle the case where the value is not
the case where the value is not defined.

