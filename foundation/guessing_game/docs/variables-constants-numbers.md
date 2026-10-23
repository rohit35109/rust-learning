# Variables, Constants, Literals, and Number Overflow

[Back to Main Notes](base.md)

Notes from Rust Book Chapter 3 (Variables and Mutability + start of Data Types).

## Book vs Reference

- **The Rust Book** = teaching document for learners.
- **The Rust Reference** = exact language rules for advanced use.

If the Book links to the Reference (example: constant evaluation), read the Book first. Open the Reference only if you need deep legal details. Skip it when it feels too hard.

Useful links:

- [Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)
- [Constant evaluation (Reference)](https://doc.rust-lang.org/reference/const_eval.html)

## Constants (`const`)

### What I learned

- Type is required: `const MAX: u32 = 100;`
- Name style: all capital letters, words separated by `_` (no spaces)
  - Example: `THREE_HOURS_IN_SECONDS`
- Value must be known at **compile time**
- Value never changes (`mut` is not allowed with `const`)

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // compiler calculates 10800 at build time
```

### Rust `const` vs JS/TS `const`

| | JS/TS `const` | Rust `let` (no mut) | Rust `const` |
|---|---|---|---|
| Can reassign the name? | No | No | No |
| Object insides can change? | Often yes | Depends | Value is fixed |
| Value from user input / runtime? | Yes | Yes | No |
| Type must be written? | No | No (can infer) | Yes |
| When is value decided? | Runtime | Runtime | Compile time |

Simple rule: Rust `const` is a fixed compile-time value. JS/TS `const` mostly means “do not reassign this name.”

## Shadowing

Shadowing means declaring a **new** variable with the same name using `let` again. The new one hides the old one.

```rust
let y = 5;
let y = y + 1; // new variable named y
```

This is **not** the same as reassignment:

```rust
let mut y = 5;
y = y + 1; // same variable, new value
```

### Shadowing vs `mut`

| | What you can do |
|---|---|
| `let mut y` then `y = ...` | Change the **value**, same **type** only |
| `let y` then `let y` again | New variable; **type can change** |

```rust
let spaces = "   ";        // text
let spaces = spaces.len(); // number — allowed with shadowing

let mut spaces = "   ";
spaces = spaces.len();     // error — mut cannot change type
```

Spelling: it is `mut`, not `mute`.

## Quick Summary: `const`, `let`, `mut`, shadowing

- **`const`**: fixed value, type required, `ALL_CAPS`, known at compile time.
- **`let`**: immutable variable; value can come from runtime.
- **`let mut`**: same variable can change value, but not type.
- **Shadowing**: new `let` with the same name; old name is hidden; type may change.

## Literals

A **literal** is a value written directly in the source code.

```rust
5          // number literal
3.14       // floating number literal
"hello"    // string literal
'A'        // character literal
true       // boolean literal
```

| Concept | Meaning | Example |
|---|---|---|
| Literal | Value written in code | `42` |
| Variable | Name that holds a value | `let x = 42;` |
| Expression | Something calculated | `x + 1` |

## Number Literals

Different ways to write numbers:

| Form | Meaning | Example | Decimal value |
|---|---|---|---|
| Decimal | Normal base-10 | `98_222` | 98222 |
| Hexadecimal | Base-16, prefix `0x` | `0xff` | 255 |
| Octal | Base-8, prefix `0o` | `0o77` | 63 |
| Binary | Base-2, prefix `0b` | `0b1111_0000` | 240 |
| Byte | One byte as `b'A'` | `b'A'` | 65 |

### Underscore `_` in numbers

`_` is only for easier reading. The compiler ignores it.

```rust
let a = 1000;
let b = 1_000;
// a and b are the same value: 1000
```

`1_000` does not mean special math. It is like writing `1,000` in English, but Rust uses `_` because `,` already has another meaning in code.

## Number Overflow

Each number type has a fixed size. Example: `u8` holds only `0` to `255`. Going past that is **overflow**.

Example: `250u8 + 10` wants `260`, but `u8` max is `255`.

Rust gives four method families to handle overflow:

### 1. `wrapping_*` — wrap around

```rust
250u8.wrapping_add(10) // 4
```

Like a clock: after max, start from 0 again.

### 2. `checked_*` — return `None` on overflow

```rust
250u8.checked_add(10) // None
10u8.checked_add(5)   // Some(15)
```

Good when overflow is an error and you want to handle it safely.

### 3. `overflowing_*` — value + true/false flag

```rust
250u8.overflowing_add(10) // (4, true)
10u8.overflowing_add(5)   // (15, false)
```

Gives the wrapped value and whether overflow happened.

### 4. `saturating_*` — stop at min or max

```rust
250u8.saturating_add(10) // 255
5u8.saturating_sub(10)   // 0
```

Do not wrap. Stick to the edge value.

### Side-by-side: `250u8 + 10`

| Method | Result | Meaning |
|---|---|---|
| `wrapping_add` | `4` | Wrap around |
| `checked_add` | `None` | Report failure |
| `overflowing_add` | `(4, true)` | Value + overflow flag |
| `saturating_add` | `255` | Clamp to max |

Mental model:

- **wrapping** = clock / circle math
- **checked** = safe or nothing (`Option`)
- **overflowing** = answer + did it overflow?
- **saturating** = stop at the wall (min/max)

For most app code, start with `checked_*` when overflow matters.

## Revision Questions

1. Can a Rust `const` get its value from user input? Why?
2. What is the difference between `y = y + 1` and `let y = y + 1`?
3. Why is `1_000` equal to `1000`?
4. What does `checked_add` return when overflow happens?
5. How is Rust `const` different from JS/TS `const`?