# Learning: Ownership & Borrowing

**Topic:** ownership_and_borrowing
**Subtopic:** Core ownership rules and borrowing
**Difficulty:** intermediate

---

# Ownership & Borrowing in Rust

Rust's ownership system ensures memory safety without garbage collection by enforcing strict rules at compile time. Understanding ownership, moves, and borrowing is essential for writing efficient and safe Rust code.

## Core Concepts

### Ownership Rules
1. **Each value has a single owner** - Only one variable owns a value at any time
2. **When owner goes out of scope, value is dropped** - Automatic memory cleanup
3. **Ownership can be moved or borrowed** - Transfer or temporary access

### Move Semantics
- **Move**: Ownership transfer from one variable to another
- **Copy**: Some types (integers, booleans) are copied instead of moved
- **Clone**: Explicit deep copying when needed

### Borrowing
- **Immutable references (`&T`)**: Multiple readers allowed
- **Mutable references (`&mut T`)**: Single writer, no other references
- **Lifetime**: Ensures references don't outlive the data they point to

## Code Examples

### Example 1: Ownership and Moves
```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2, s1 is no longer valid
    
    // println!("{}", s1); // ERROR: value borrowed after move
    println!("{}", s2); // OK: s2 owns the string
    
    let x = 5;
    let y = x; // Copy (not move) because i32 implements Copy trait
    println!("{} {}", x, y); // OK: both x and y are valid
}
```

### Example 2: Borrowing Rules
```rust
fn main() {
    let mut s = String::from("hello");
    
    // Multiple immutable references OK
    let r1 = &s;
    let r2 = &s;
    println!("{} {}", r1, r2);
    
    // Mutable reference (after immutable refs are done)
    let r3 = &mut s;
    r3.push_str(" world");
    println!("{}", r3);
    
    // ERROR: cannot have mutable and immutable refs simultaneously
    // let r4 = &s; // This would fail if r3 was still used after
}

fn calculate_length(s: &String) -> usize {
    s.len() // Borrowing: function can read but doesn't take ownership
}
```

### Example 3: Lifetimes
```rust
// Explicit lifetime annotation ensures return reference is valid
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string");
    {
        let string2 = String::from("xyz");
        let result = longest(&string1, &string2);
        println!("Longest: {}", result); // OK: both references valid here
    }
    // string2 dropped here, but that's fine since we're not using result
}
```

## Common Pitfalls

### 1. **Use After Move**
```rust
let s1 = String::from("hello");
let s2 = s1;
println!("{}", s1); // ERROR: s1 moved
```
**Solution**: Use `clone()` if you need both values, or borrow with `&s1`

### 2. **Dangling References**
```rust
fn dangle() -> &String { // ERROR: missing lifetime specifier
    let s = String::from("hello");
    &s // s goes out of scope, reference becomes invalid
}
```
**Solution**: Return owned value `String` instead of reference

### 3. **Multiple Mutable References**
```rust
let mut s = String::from("hello");
let r1 = &mut s;
let r2 = &mut s; // ERROR: cannot borrow as mutable more than once
```
**Solution**: Ensure mutable references don't overlap in scope

### 4. **Mixing Mutable and Immutable References**
```rust
let mut s = String::from("hello");
let r1 = &s;
let r2 = &mut s; // ERROR: cannot borrow as mutable while immutable borrow exists
println!("{}", r1);
```
**Solution**: Finish using immutable references before creating mutable ones