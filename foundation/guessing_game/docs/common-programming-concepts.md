# Common Programming Concepts

[Back to Main Notes](base.md)

Notes from Rust Book Chapter 3.  
Related: [Variables, Constants, Literals, and Number Overflow](variables-constants-numbers.md)

---

## 1. Variables and Mutability

- By default, variables are **immutable** (cannot change).
- Use `mut` when you need to change the value.

```rust
let mut x = 5;
x = 6; // allowed because of mut
```

- `mut` lets you change the **value**, not the **type**.

---

## 2. Constants (`const`)

- Type is required.
- Name style: `ALL_CAPS_WITH_UNDERSCORES` (no spaces).
- Value must be known at **compile time**.
- Cannot use `mut`.

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

### Rust `const` vs JS/TS `const`

| | JS/TS `const` | Rust `let` | Rust `const` |
|---|---|---|---|
| Reassign name? | No | No (unless `mut`) | No |
| Value from runtime / user input? | Yes | Yes | No |
| Type required? | No | No (can infer) | Yes |
| When decided? | Runtime | Runtime | Compile time |

---

## 3. Shadowing

Declaring again with `let` creates a **new** variable with the same name. The new one hides the old one.

```rust
let y = 5;
let y = y + 1; // new variable
```

| | Meaning |
|---|---|
| `let mut y` then `y = ...` | Same variable, same type, new value |
| `let y` then `let y` again | New variable; type **can** change |

```rust
let spaces = "   ";        // text
let spaces = spaces.len(); // number — allowed
```

---

## 4. Data Types

Rust is statically typed. Types are known at compile time.

### Scalar types (single value)

#### Integers

- Signed: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`
- Unsigned: `u8`, `u16`, `u32`, `u64`, `u128`, `usize`
- Default integer type is often `i32`
- `u32` = unsigned 32-bit (0 and positive only)
- `i32` = signed 32-bit (negative and positive)

#### Number literals

A **literal** is a value written directly in code (`5`, `"hi"`, `true`).

| Form | Example |
|---|---|
| Decimal | `98_222` |
| Hex | `0xff` |
| Octal | `0o77` |
| Binary | `0b1111_0000` |
| Byte | `b'A'` |

`_` inside numbers is only for reading. `1_000` equals `1000`.

#### Overflow methods

For `250u8 + 10` (overflow):

| Method | Result | Meaning |
|---|---|---|
| `wrapping_add` | `4` | Wrap around |
| `checked_add` | `None` | Fail safely |
| `overflowing_add` | `(4, true)` | Value + flag |
| `saturating_add` | `255` | Stop at max |

#### Floating point

- `f32`, `f64` (default is usually `f64`)
- Integer ÷ integer truncates: `52 / 5` → `10`
- For decimals: `52.0 / 5.0` → `10.4`
- Do not mix integer and float in one operation without converting

#### Boolean

```rust
let boolean_value = true;
```

Rust does **not** convert numbers to boolean like JS (`if (1)`). Condition must be a real `bool`.

#### Character

```rust
let c = 'x';
let heart = '🥳'; // char uses single quotes
```

`char` is 4 bytes and can hold Unicode.

---

## 5. Compound Types

### Tuple

Fixed length. Mixed types allowed. Use **parentheses `()`**.

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
let (x1, x2, x3) = tup; // destructuring
let five_hundred = tup.0; // index access
```

Empty tuple `()` is called **unit**.

### Array

Fixed length. All elements same type. Stored on the stack.

```rust
let a: [i32; 4] = [1, 2, 3, 4];
let b = [3; 5]; // [3, 3, 3, 3, 3]
let first = a[0];
```

- Index type is often `usize`
- Invalid index → **panic** at runtime
- Safer option later: `.get()`

---

## 6. Functions

```rust
fn parameterised_function(x: i32, c: char) {
    println!("{x}, {c}");
}

fn return_five() -> i32 {
    5 // no semicolon = return this value
}
```

- Parameter types must be written
- Return type uses `-> Type`
- Common style: last expression, no `;`
- Early exit: `return value;`

---

## 7. Statements and Expressions

| | Meaning | Example |
|---|---|---|
| Statement | Does work, does not return a value | `let x = 1;` |
| Expression | Produces a value | `1 + 1`, function call, block `{ ... }` |

```rust
let exp = {
    let y = 3;
    let z = 3;
    y + z // no semicolon → expression, value goes to exp
};
```

If you write `y + z;`, it becomes a statement and does not return the number.

Rust does not allow JS-style `x = y = 2` with `let`, because `let` is a statement.

---

## 8. Comments

| Syntax | Use |
|---|---|
| `//` | Line comment |
| `/* ... */` | Block comment |
| `///` | Doc comment for the next item |
| `//!` | Doc comment for the file/module |

`cargo doc` can build HTML docs from `///` and `//!`.

---

## 9. Control Flow

### `if` / `else if` / `else`

```rust
if x < 5 {
    println!("true");
} else if x > 5 && x < 15 {
    println!("middle");
} else {
    println!("false");
}
```

- Condition must be `bool`
- `if` is an expression, so it can return a value:

```rust
let number = if condition { 5 } else { 10 };
// both arms must be the same type
```

### `loop`

Repeats forever until `break`. Can return a value:

```rust
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2;
    }
};
```

### Loop labels

Useful with nested loops:

```rust
'counting_up: loop {
    loop {
        break;                 // breaks inner loop
        break 'counting_up;    // breaks outer labeled loop
    }
}
```

### `while`

```rust
while number != 0 {
    number -= 1;
}
```

### `for`

Best way to loop over a collection:

```rust
for element in a {
    println!("{element}");
}

for number in (1..4).rev() {
    println!("{number}");
}
```

- `1..4` = 1, 2, 3 (end not included)
- `.rev()` = reverse order
- Prefer `for` over manual index `while` (safer and clearer)

### Loop summary

| Loop | Meaning |
|---|---|
| `loop` | Forever, until `break` |
| `while` | While condition is true |
| `for` | Over a collection or range |

---

## 10. Quick Revision Questions

1. Can `mut` change a variable’s type?
2. Why must a `const` value be known at compile time?
3. Why is `52 / 5` equal to `10`?
4. Tuple uses `( )` or `{ }`?
5. What happens if you access an invalid array index?
6. Why does the last line in a returning function usually have no `;`?
7. Can you write `if 1 { ... }` in Rust like in JS?
8. How do you break an outer loop from an inner loop?
9. Why is `for` often better than `while` with a manual index?

---

## Book Link

[Common Programming Concepts](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html)