# Learning: Ownership & Borrowing

**Topic:** ownership_and_borrowing
**Subtopic:** Core ownership rules and borrowing
**Difficulty:** intermediate

---

# Ownership & Borrowing in Rust

Rust's ownership system ensures memory safety without a garbage collector by enforcing strict rules at compile time. Understanding ownership, borrowing, and lifetimes is crucial for writing safe and efficient Rust code.

## Core Concepts

### Ownership Rules
1. **Each value has a single owner** - Only one variable can own a value at a time
2. **Values are dropped when owner goes out of scope** - Automatic cleanup prevents memory leaks
3. **Ownership can be moved or borrowed** - Transfer ownership or temporary access

### Move Semantics
- Assignment transfers ownership by default for heap-allocated data
- Original variable becomes invalid after move
- Stack data (Copy types) is duplicated instead

### Borrowing
- **Immutable references (`&T`)** - Multiple allowed, no mutation
- **Mutable references (`&mut T`)** - Only one at a time, exclusive access
- References must always be valid (no dangling pointers)

### Lifetimes
- Ensure references don't outlive the data they point to
- Usually inferred by compiler, sometimes need explicit annotation
- Prevent use-after-free bugs at compile time

## Code Examples

### Example 1: Ownership and Moves
```rust
fn main() {
    let s1 = String::from("hello");  // s1 owns the string
    let s2 = s1;                     // Ownership moves to s2
    // println!("{}", s1);           // Error! s1 no longer valid
    
    let s3 = s2.clone();             // Explicit deep copy
    println!("{} {}", s2, s3);       // Both valid - s3 has its own data
    
    takes_ownership(s2);             // s2 moved into function
    // println!("{}", s2);           // Error! s2 no longer valid
}

fn takes_ownership(s: String) {      // s comes into scope
    println!("{}", s);
}   // s goes out of scope and is dropped
```

### Example 2: Borrowing Rules
```rust
fn main() {
    let mut s = String::from("hello");
    
    // Multiple immutable references OK
    let r1 = &s;
    let r2 = &s;
    println!("{} {}", r1, r2);       // Both references used here
    
    // Mutable reference after immutable refs go out of scope
    let r3 = &mut s;                 // OK - no other active references
    r3.push_str(" world");
    println!("{}", r3);
    
    // Cannot have mutable and immutable refs simultaneously
    // let r4 = &s;                  // Error! Cannot borrow as immutable
    
    calculate_length(&s);            // Pass by reference (borrowing)
}   // s still valid here - ownership never transferred

fn calculate_length(s: &String) -> usize {  // Borrows, doesn't take ownership
    s.len()
}   // s reference goes out of scope, but doesn't drop the String
```

### Example 3: Lifetimes
```rust
fn main() {
    let string1 = String::from("long string");
    let string2 = "short";
    
    let result = longest(&string1, string2);
    println!("Longest: {}", result);
}

// Lifetime annotation ensures returned reference is valid
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x                            // Return reference with lifetime 'a
    } else {
        y                            // Both inputs must live at least as long as 'a
    }
}

// Struct with lifetime parameter
struct ImportantExcerpt<'a> {
    part: &'a str,                   // Reference must live as long as struct
}
```

## Common Pitfalls

### 1. **Use After Move**
```rust
let s1 = String::from("hello");
let s2 = s1;
println!("{}", s1); // Error! Value moved to s2
```

### 2. **Borrowing Conflicts**
```rust
let mut s = String::from("hello");
let r1 = &s;
let r2 = &mut s; // Error! Cannot borrow as mutable while immutable ref exists
```

### 3. **Dangling References**
```rust
fn dangle() -> &String { // Error! Missing lifetime specifier
    let s = String::from("hello");
    &s // Returns reference to value that will be dropped
}
```

### 4. **Lifetime Confusion**
```rust
let r;
{
    let x = 5;
    r = &x; // Error! x doesn't live long enough
}
println!("{}", r); // x already dropped here
```

**Key Takeaway**: The borrow checker prevents data races and memory safety issues at compile time. When in doubt, prefer borrowing over moving, and let the compiler guide you toward safe solutions.