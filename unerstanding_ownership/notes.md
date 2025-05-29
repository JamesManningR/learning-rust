# Understanding Ownership

Ownership is a core concept of rust. It is arguably it's defining feature.

## What is ownership?

In Rust, variables have a lifespan tied to ownership. When a variable goes out of
scope or ownership is transferred, the memory is automatically freed.

This allows Rust to solve key problems:

- **No Garbage Collection** - Since the ownership model automatically frees memory
  when it's no longer needed, there's no need for a garbage collector
- **Memory Safety** - The ownership rules prevent common bugs like use-after-free,
  double-free, and data races at compile time
- **Predictable Performance** - No surprise GC pauses or hidden allocations

> [!HINT] Terms
>
> - **Use after free** - accessing the memory after its been deallocated (causes
>   unknown behaviour)
> - **double-free** - Deallocating the same memory twice (causes crash)
> - **data-race** - Multiple threads accessing the same memory simultaneously, at
>   least one of which is writing
> - **gc pauses** - When the garbage collector (gc) pauses the program to fee up
>   memory

## How does ownership work?

Ownership is how Rust manages memory on the stack and heap. A variable owns
a value, which may be stored on the stack (like integers) or heap (like String data)
When the variable goes out of scope, the owned value is dropped and its memory
is freed.

Ownership can be transferred, but this is a one way operation. Returning of ownership
must be explicit.

```rust
let x = String::from("hello");
let y = x;  // ownership transfers from x to y

println!("{x}") // Not valid
// x is now invalid, y owns the String

// -- OR --

fn take_and_return(s: String) -> String {
    s  // explicitly return ownership back
}

let a = String::from("hello");
let b = take_and_return(a);  // a gives ownership, function returns it to b

// -- OR --

let x = String::from("hello");
let y = x;           // x -> y
let z = y;           // y -> z (y is now invalid)
// To "return" to x, you'd need: let x = z;
```

You can also `borrow` a value by passing a reference. This allows you to use the
value without taking ownership of it.

```rust
fn calculate_length(s: &String) -> usize {
    s.len()
}

let x = String::from("hello");
let len = calculate_length(&x);  // borrow, don't move
// x is still valid here
```

> [!ATTENTION] Rules of Borrowing
> At any given time, you can have either:
>
> - Multiple immutable references `&`
> - A single mutable reference `&mut`
>
> But never both simultaneously. Violating these rules will result in a compile error.
