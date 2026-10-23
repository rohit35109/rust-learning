
[Back to Main Notes](base.md)

## Guessing Game - Summary of Concepts Learned

### 1. `fn main()`

- `main` is the starting point of a Rust program.
- Rust starts running the program from `main`.

### 2. `println!` - Macro

- `println!` is used to print text.
- The `!` sign means it is a **macro**, not a normal function.
- A **function** runs at runtime. It receives values.
- A **macro** runs at compile time. It reads the code and creates new code from it. The created code runs at runtime.
- A normal Rust function has a fixed number of arguments. A macro can take a different number of arguments. Example: `println!("{}", a, b, c)`.
- A macro can also check the format string at compile time. If arguments are missing, Rust gives a compile error.
- Rule: If a function can do the work, use a function. Use a macro only when a function cannot do the work.
- Other macros: `format!`, `vec!`, `panic!`, `dbg!`.

### 3. Crate, Cargo, and `rand`

- **Cargo** is the Rust tool to create projects, build code, and manage libraries. It is like `npm` for Node.js.
- A **crate** is a Rust library or program unit. It is like a package.
- `rand` is a crate used to create random numbers.
- Libraries are listed in `Cargo.toml`. Public libraries are available at `crates.io`.
- `rand::rng()` is a function. It gives a random number generator object.
- `.random_range(1..=100)` is a **method** called on that object. A method is a function attached to a type or object.
- `1..=100` means numbers from 1 to 100, including 100.

### 4. How to Find Things in the Standard Library

- `use std::cmp::Ordering;` means: take `Ordering` from the `cmp` module in the standard library (`std`).
- To learn what exists in the standard library:
  - Search at `doc.rust-lang.org/std`
  - Use Go to Definition in the editor
  - Run `rustup doc --std` to open local documents

### 5. Variables: `let` and `mut`

- `let` creates a variable. In Rust this is called a **binding**.
- By default, a variable cannot be changed. This is called **immutable**.
- `let mut guess` means the value of `guess` can be changed. This is called **mutable**.
- Changing a value is allowed only with the **same type**. A `String` variable can only receive another `String`. It cannot receive a number.
- Using `let` again with the same name creates a **new variable** with the same name. This is called **shadowing**.
- With shadowing, the type can change:

```rust
let mut guess = String::new();   // String
let guess: u32 = guess.trim().parse().expect("Please type a number"); // u32
```

- The old variable is not changed. A new variable takes the same name.
- In memory, the old value and new value are separate values. The new variable is not a shared reference to the old value. When the old value is no longer needed, Rust removes it. Rust does not use a garbage collector.

### 6. Types and `u32`

- `u32` means **unsigned 32-bit integer**.
  - `u` = unsigned = only zero and positive numbers.
  - `32` = uses 32 bits = 4 bytes.
  - Range: 0 to 4,294,967,295.
- `i32` means signed 32-bit integer. It can store negative numbers also.
- Common types:
  - Integer: `i32`, `u32`, `i64`, `u64`
  - Decimal number: `f32`, `f64` (Rust has no type named `double`. `f64` is the same level of precision as `double` in other languages.)
  - True/false: `bool`

### 7. `String::new()`

- `String::new()` creates an empty string.
- It can grow when we add text to it.

### 8. Reading User Input and Borrowing

```rust
let mut guess = String::new();
io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
```

- `io::stdin()` gives access to user input from the keyboard.
- `read_line` reads one line and writes it into `guess`.
- `&mut guess` means we **lend** `guess` to `read_line` for a short time, with permission to change it. This lending is called **borrowing**.
- `guess` is still the owner. After `read_line` finishes, control comes back to us.
- Why both `mut` words?
  - `let mut guess` = this variable is allowed to change.
  - `&mut guess` = at this place, we are lending it with permission to change.
- `&guess` means lending only for reading. `&mut guess` means lending for reading and writing.
- `read_line` also adds a new line character at the end. For this reason we use `.trim()` before converting to a number.

### 9. Converting String to Number with `parse()`

- `parse()` does not always create an integer. It creates the type we ask for.
- The type is given on the left side:

```rust
let n: u32 = "42".parse().expect("not a number");
let f: f64 = "3.14".parse().expect("not a number");
let b: bool = "true".parse().expect("not a bool");
```

- Another way to give the type:

```rust
let n = "42".parse::<u32>().expect("not a number");
```

### 10. `Result`, `Ok`, and `Err`

- `parse()` and `read_line()` can fail. So they return a `Result`.
- `Result` has two possible values:
  - `Ok(value)` = work completed. The value is inside it.
  - `Err(error)` = work failed. Error details are inside it.
- This is like a box with two possible forms. We must open the box and check which form it is.

### 11. `expect()` is Not a Catch Block

- `.expect("message")` means: if the result is `Ok`, give me the value. If the result is `Err`, stop the program with this message. Stopping like this is called a **panic**.
- So `expect` does not handle the error. It crashes the program on error.
- Handling the error and continuing is done with `match`:

```rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
```

- Here, wrong input does not crash the program. It starts the loop again.

### 12. `match` and Match Arms

- `match` checks a value and runs code based on its form. It is like a `switch` statement, but stronger.
- `match` must cover all possible cases. If a case is missing, Rust gives a compile error.
- One line inside `match` is called an **arm**. An arm has two parts:

```text
pattern => code to run
```

- The left part is the **pattern**. The right part is the code.
- The first matching arm runs.
- Common patterns:
  - `Ok(num)` = if it is `Ok`, take the inside value and store it in `num`.
  - `Err(_)` = if it is `Err`, ignore the error details. `_` means "I do not need this value."
  - `Ordering::Less` = only this exact value.
  - `_` = matches anything.

### 13. Comparing Numbers with `.cmp()` and `Ordering`

- `.cmp()` is a **method** on numbers. It compares two numbers.
- It returns a value of type `Ordering` from `std::cmp`.
- `Ordering` has three values:
  - `Ordering::Less` = first number is smaller.
  - `Ordering::Equal` = both numbers are the same.
  - `Ordering::Greater` = first number is bigger.
- In general English, "ordering" means the result of comparing: less, equal, or greater.

```rust
match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small."),
    Ordering::Greater => println!("Too Large."),
    Ordering::Equal => println!("You Win!!"),
}
```

- `&secret_number` means `cmp` only reads the value. It does not take ownership of it.

### 14. Loops in Rust

- `loop { }` = repeat again and again. It does not stop by itself. We stop it with `break`. It is like `while (true)` in JavaScript.
- `while condition { }` = repeat until the condition becomes false.
- `for x in 1..=100 { }` = repeat for each value in a list or range.
- `break` = stop the loop.
- `continue` = skip the remaining code in this round and start the next round of the loop.

### 15. Full Flow of the Guessing Game Program

1. `main` starts.
2. The `rand` crate creates a secret number between 1 and 100.
3. An infinite `loop` starts.
4. The program creates an empty mutable `String` named `guess`.
5. It lends `guess` to `read_line` with `&mut`, so user input is written into it.
6. `.trim()` removes extra spaces and the new line character.
7. `.parse()` tries to convert the text into `u32`.
8. `match` checks the `Result`:
   - `Ok(num)` = the number is stored in a new `u32` variable named `guess` (shadowing).
   - `Err(_)` = `continue` starts the loop again and asks for input again.
9. `.cmp()` compares `guess` with `secret_number`.
10. `match` prints Too small / Too Large / You Win. On winning, `break` stops the loop.

### Quick Questions for Revision

1. Why do we write `&mut guess` if we already wrote `let mut guess`?
2. What is the difference between `loop` and `for`?
3. What are the two possible results of `parse()`?
4. Why can we use the name `guess` for both a `String` and a `u32`?
5. Does `expect` handle an error or crash the program?
