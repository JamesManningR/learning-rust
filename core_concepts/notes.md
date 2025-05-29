# Core Concepts

## Variables and Mutability

### Key Terms

- `let` - Keyword to declare an immutable variable binding in Rust
- `mut` - Keyword that makes a variable binding mutable
- `const` - Keyword for compile-time constants that never change
- `mutable` - Able to be changed or modified after creation
- `immutable` - Cannot be changed or modified after creation
- `shadowing` - Declaring a new variable with the same name, hiding the previous
  one
- `closure` - Anonymous function that can capture variables from surrounding scope
- `compile time` - When code is translated from source to executable, before running
- `runtime` - When the compiled program is actually executing

Variables are by default immutable.
You can think of a `const` as a compile time calculation / assignment
`let` without the `mut` is the *equivalent* of a js `const`
`let mut` is the *equivalent* of a js `let`

> [!note] Some Differences
> A `let` in rust can be *shadowed*, which means it can be re-declared. This is
> the same behaviour as js when the variable is in a different scope / closure.
> When in the same scope, the behaviour differs.
>
> In JS, if a let variable is assigned in the same scope with the same name, it
> throws an error: in rust it allows the re-declaration and discards the old
> variable.

## Data Types

Variables have many types in Rust. Rust allows for low level control for data
types, and so it exposes many of them.

### Scalar Types

Scalar types represent a single value as apposed to a collection.

#### Integers

| Type    | Size           | Signed/Unsigned |
|---------|----------------|-----------------|
| `i8`    | 8-bit          | Signed          |
| `u8`    | 8-bit          | Unsigned        |
| `i16`   | 16-bit         | Signed          |
| `u16`   | 16-bit         | Unsigned        |
| `i32`   | 32-bit         | Signed          |
| `u32`   | 32-bit         | Unsigned        |
| `i64`   | 64-bit         | Signed          |
| `u64`   | 64-bit         | Unsigned        |
| `i128`  | 128-bit        | Signed          |
| `u128`  | 128-bit        | Unsigned        |
| `isize` | Arch-dependent | Signed          |
| `usize` | Arch-dependent | Unsigned        |

> [!note] Note
> the Signed 32-bit `i32` is the default for integers

#### Floating Point

| Type    | Size           | Signed/Unsigned |
|---------|----------------|-----------------|
| `f32`   | 32-bit         | Signed          |
| `f64`   | 64-bit         | Signed          |

> [!note] Note
> the Floating point 64-bit `f64` is the default for floating point numbers

#### Other

| Type   | Size    | Description        |
|--------|---------| -------------------|
| `bool` | 1-byte  | true or false      |
| `char` | 4-bytes | Unicode character  |

### Compound Types

Compound types are types that can group multiple values into one type.

| Type     | Syntax                    | Fixed Size | Mixed Types  | Access Method    |
|----------|---------------------------|------------| -------------|------------------|
| Tuple    | `(i32, f64, u8)`          | Yes        | Yes          | `.0`, `.1`, etc. |
| Array    | `[i32; 5]`                | Yes        | No           | `[0]`, `[1]`, etc|
| String   | `String::from("text")`    | No         | No           | Methods          |
| &str     | `"string literal"`        | Yes        | No           | Methods          |

>[!note] What's the difference between an `&str` (string literal) and a `String`?
> A string is mutable in size whereas string literal must be known at runtime

>[!note] How can a string literal be mutable?
> It's not mutable in the intuitive sense, the thing that is mutable is the
> variable binding. You can modify which location in memory the string literal
> refers to, but not the literal `contents` itself.

## Functions

Functions are fairly simple, they may take args and return a single value

### Syntax

```rust
fn foo(bar: i32, baz: char) {
    println!("{bar}: {baz}")
}
```

### Things to note

- Function args must be typed
- Functions declarations may go before or after their caller, as long as they
  are defined in the same or higher scope
- The final expression, without a `;` is returned
- Values can also be returned using the `return` keyword

> [!note] Decleration collection / name resolution
> Rust scans the entire scope for declarations first and then resolves calls.
> This means you can do some strange looking things

> ```rs
> fn main() {
>   foo(); // Works due to function declaration scanning
>   fn foo() { println!("hi"); }
> }
> ```

## Control Flow

### If

If statements take in an expression and continues if true

```rs
let number = 3;

if number < 5  {
    println!("Number is less than 5");
}
```

> [!CAUTION] if expressions
> Expressions **must** resolve to a bool. No implicit truthy or falsy values.

#### else if

If you have more cases, you can use an else if expression

```rs
if number % 4 == 0 {
    println!("Number is divisible by 4");
} else if number % 3 == 0 {
    println!("Number is divisible by 3");
} else if number % 2 == 0 {
    println!("Number is divisible by 2");
}
```

Note this will only ever choose 1 path. Once one of them is satisfied, the
rest of the block will be skipped

#### else

Finally if you want a default case (if all previous cases were not matched)
You can use the `else` keyword

```rs
if number % 4 == 0 {
    println!("Number is divisible by 4");
} else if number % 3 == 0 {
    println!("Number is divisible by 3");
} else if number % 2 == 0 {
    println!("Number is divisible by 2");
} else {
    println!("Number is not divisible by 2, 3 or 4");
}
```

#### If expression

You can use an if statement directly in a variable asignemnt, as if a ternary
operator

```rs
// You can use if in a let statement
let is_even = if number % 2 == 0 { true } else { false };
println!("is_even: {}", is_even)
```

### Match Expressions

Match expressions are a powerful tool which allow you to define a list of patterns
to match against a value, which would otherwise be expressed in a large list of
if expressions.

```rs
match (id + 1).cmp(&last_played_line_id) {
    std::cmp::Ordering::Less => Line::styled(line, ui.theme.lyrics_played()),
    std::cmp::Ordering::Equal => Line::styled(line, ui.theme.lyrics_playing()),
    _ => Line::raw(line),
```

Note that matches are expressions, so they return a value at the end of their
execution. The different branches of each case are known as *arms*

### Loops

#### Loop

Loops using the `loop` keyword must be explicitly exited using the `break`
keyword.

```rs
let mut counter = 0;

loop {
    println!("And another one {counter}"); // 0,1,2,3,[...],10
    counter += 1;

    if counter >= 10 {
        break;
    };
}
```

`loop`s can also return values by adding a value after the `break` keyword

```rs
counter = 0;

let result = loop {
    counter += 1;

    if counter == 10 {
        break counter * 2;
    }
};
println!("This should equal 20: {result}"); // 20
```

loops can also be labeled. This allows you to break specific loops in your code

```rs
    // You can optionally label a loop using a ' prefix
    // so you can use the break or continue keyword for a specific loop
    // This is used for nested loops
    let mut count = 0;
    'count_up: loop {
        println!("count: {count}");
        let mut remaining = 10;

        loop {
            println!("remaining: {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'count_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
```

#### While

This loop will run and check a condition at the end of every iteration.

```rs
let mut number = 3;

while number != 0 {
    println!("before: {number}"); // 3,2,1
    number -= 1;
    println!("after: {number}"); // 2,1,0
}
```

#### For loops

For loops allow you to iterate through a collection or range using an iterator.

```rs
fn for_loops() {
    let arr = [10, 20, 30, 40, 50];

    for el in arr {
        println!("The value is: {el}");
    }

    // We can also loop over a range using the range util
    // Not that this is not inclusive
    for num in (1..4) {
        println!("{num}")
    }

    // and in reverse
    for num in (1..4).rev() {
        println!("{num}")
    }
}
```
