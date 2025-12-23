# MCQs: Ownership & Borrowing
**Topic:** ownership_and_borrowing
**Difficulty:** intermediate
**Date:** 2025-12-23

---

## Question 1
What happens when the following Rust code is compiled and executed?

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    let s3 = s1.clone();
    println!("{}", s1);
}
```

- [ ] A) Prints "hello" successfully
- [ ] B) Compilation error: borrow of moved value `s1`
- [ ] C) Runtime panic due to double free
- [ ] D) Compilation error: cannot find value `s1` in this scope

**Concept tested:** Move semantics and ownership transfer

## Question 2
Which of the following borrowing scenarios will compile successfully?

```rust
fn main() {
    let mut data = vec![1, 2, 3];
    // Scenario options below
}
```

- [ ] A) `let r1 = &data; let r2 = &mut data; println!("{:?} {:?}", r1, r2);`
- [ ] B) `let r1 = &mut data; let r2 = &mut data; r1.push(4);`
- [ ] C) `let r1 = &data; let r2 = &data; println!("{:?} {:?}", r1, r2);`
- [ ] D) `let r1 = &mut data; let r2 = &data; r1.push(4);`

**Concept tested:** Borrowing rules and mutable/immutable reference coexistence

## Question 3
In the context of lifetimes, what does the following function signature indicate?

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

- [ ] A) The returned reference will live as long as the longer of the two input lifetimes
- [ ] B) The returned reference will live as long as the shorter of the two input lifetimes  
- [ ] C) All parameters and return value must have exactly the same lifetime
- [ ] D) The function creates a new lifetime 'a that extends beyond the input parameters

**Concept tested:** Lifetime annotations and lifetime relationships