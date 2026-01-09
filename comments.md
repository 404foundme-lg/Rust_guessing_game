Here are several practical ways to work with comments in **Rust**, from basic usage to tooling and documentation-related patterns.

---

## 1. Line Comments (`//`)

Used for short explanations or notes.

```rust
fn main() {
    // This prints a greeting
    println!("Hello, world!");
}
```

**Notes**

* Everything after `//` on the same line is ignored by the compiler.
* Most common comment type.

---

## 2. Block Comments (`/* ... */`)

Used for longer explanations or temporarily disabling code.

```rust
/*
This is a block comment.
It can span multiple lines.
*/
fn main() {
    println!("Hello!");
}
```

### Nested Block Comments (Rust-specific feature)

Rust allows block comments to be nested:

```rust
/*
Outer comment
    /* Inner comment */
Still inside outer comment
*/
```

This is helpful when commenting out code that already contains comments.

---

## 3. Documentation Comments (`///`)

Used to generate API documentation with `rustdoc`.

````rust
/// Adds two numbers together.
///
/// # Examples
///
/// ```
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
fn add(a: i32, b: i32) -> i32 {
    a + b
}
````

**Key points**

* Apply to the item *below* the comment.
* Markdown is supported.
* Code blocks can be tested with `cargo test`.

---

## 4. Inner Documentation Comments (`//!`)

Used to document a module or crate as a whole.

```rust
//! This crate provides math utilities.
//! It is designed to be fast and simple.

pub fn square(x: i32) -> i32 {
    x * x
}
```

**Difference**

* `///` → documents the next item
* `//!` → documents the enclosing module or crate

---

## 5. Attribute-Style Documentation (`#[doc = "..."]`)

Less common, but useful for macros or conditional docs.

```rust
#[doc = "Returns the cube of a number."]
fn cube(x: i32) -> i32 {
    x * x * x
}
```

You can also use it conditionally:

```rust
#[cfg_attr(feature = "unstable", doc = "This function is unstable.")]
fn experimental() {}
```

---

## 6. Commenting Out Code (Temporary Debugging)

Block comments are often used to disable code quickly:

```rust
fn main() {
    /*
    let x = expensive_computation();
    println!("{}", x);
    */
}
```

**Tip**: For large sections, consider using feature flags or `cfg` attributes instead.

---

## 7. TODO, FIXME, and NOTE Comments

Common conventions recognized by editors and IDEs.

```rust
// TODO: Handle overflow
// FIXME: This panics on empty input
// NOTE: Assumes input is sorted
```

Many tools can highlight or track these automatically.

---

## 8. Linting and Documentation Tools

* `cargo doc` → Generate HTML documentation from `///` and `//!`
* `cargo clippy` → Warns about poor comment practices
* `rustfmt` → Preserves comment placement and formatting

---

## 9. Comments vs. Self-Documenting Code

Rust style encourages **clear naming** and **types** over excessive comments.

```rust
// Bad
let x = 86400; // seconds in a day

// Better
let seconds_per_day = 86_400;
```

---

### Summary Table

| Comment Type | Syntax           | Purpose                   |
| ------------ | ---------------- | ------------------------- |
| Line         | `//`             | Short notes               |
| Block        | `/* */`          | Long or nested comments   |
| Doc (outer)  | `///`            | API documentation         |
| Doc (inner)  | `//!`            | Module/crate docs         |
| Attribute    | `#[doc = "..."]` | Advanced/conditional docs |

If you want, I can also show **best practices**, **common mistakes**, or how comments interact with **macros and procedural macros** in Rust.






## 1. `TODO` — Planned Improvements or Missing Features

Used when something is intentionally incomplete.

### Example: Adding error handling later

```rust
fn parse_age(input: &str) -> u8 {
    // TODO: Return Result instead of panicking
    input.parse().unwrap()
}
```

### Example: Performance optimization

```rust
fn find_user(id: u64) -> Option<User> {
    // TODO: Replace linear search with HashMap lookup
    users.iter().find(|u| u.id == id).cloned()
}
```

### Example: Feature flag work

```rust
fn encrypt(data: &[u8]) -> Vec<u8> {
    // TODO: Implement AES-GCM encryption
    data.to_vec()
}
```

---

## 2. `FIXME` — Known Bugs or Incorrect Behavior

Used when the code **works incorrectly or dangerously**.

### Example: Panic on edge case

```rust
fn average(nums: &[i32]) -> f64 {
    // FIXME: Panics when nums is empty
    let sum: i32 = nums.iter().sum();
    sum as f64 / nums.len() as f64
}
```

### Example: Incorrect logic

```rust
fn is_even(n: i32) -> bool {
    // FIXME: This is incorrect for negative numbers
    n % 2 == 0
}
```

### Example: Race condition

```rust
fn increment(counter: &mut i32) {
    // FIXME: Not thread-safe
    *counter += 1;
}
```

---

## 3. `NOTE` — Important Assumptions or Design Context

Used to explain *why* something is written a certain way.

### Example: Input constraints

```rust
fn binary_search(nums: &[i32], target: i32) -> Option<usize> {
    // NOTE: `nums` must be sorted in ascending order
    nums.binary_search(&target).ok()
}
```

### Example: Safety invariant

```rust
unsafe fn get_unchecked(slice: &[i32], index: usize) -> i32 {
    // NOTE: Caller must guarantee index < slice.len()
    *slice.get_unchecked(index)
}
```

### Example: Platform-specific behavior

```rust
fn temp_dir() -> &'static str {
    // NOTE: Assumes Unix-like filesystem
    "/tmp"
}
```

---

## 4. Combined Usage in Real Code

```rust
fn process_data(data: &[i32]) -> i32 {
    // NOTE: data is guaranteed non-empty by caller
    // TODO: Support streaming input
    // FIXME: Overflows on large values

    data.iter().sum()
}
```

---

## 5. Common Conventions & Tooling

### IDE & Tool Support

Most editors highlight these automatically:

* `TODO`
* `FIXME`
* `NOTE`
* `HACK`
* `XXX`

### Searching from CLI

```bash
rg "TODO|FIXME|NOTE" src/
```

### Enforcing with Clippy (example policy)

```rust
// FIXME comments should not exist in release code
```

(Some teams even fail CI if `FIXME` remains.)

---

## 6. Style Tips

| Tag     | When to Use                  | Avoid When                 |
| ------- | ---------------------------- | -------------------------- |
| `TODO`  | Planned, non-broken work     | The code is actually wrong |
| `FIXME` | Known bug or unsafe behavior | It’s merely an enhancement |
| `NOTE`  | Important assumptions        | Restating obvious code     |

---

### Quick Rule of Thumb

* **Broken now?** → `FIXME`
* **Missing later?** → `TODO`
* **Important context?** → `NOTE`

If you want, I can also show how to **turn these into tracked issues**, **enforce rules with Clippy**, or **structure comments for unsafe code** in Rust.
