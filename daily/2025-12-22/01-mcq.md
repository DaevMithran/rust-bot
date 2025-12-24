# MCQs: Ownership & Borrowing
**Topic:** ownership_and_borrowing
**Difficulty:** intermediate
**Date:** 2025-12-22

---

## Question 1
What happens when the following code is compiled and run?

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
Which of the following function signatures will compile successfully in Rust?

- [ ] A) `fn process_string(s: &mut String, t: &mut String) -> &String`
- [ ] B) `fn process_string(s: &String) -> &str`
- [ ] C) `fn process_string(s: String) -> &str`
- [ ] D) `fn process_string<'a>(s: &'a String, t: &String) -> &'a str`

**Concept tested:** Lifetimes and borrowing rules in function signatures

## Question 3
Given the following code, which statement about borrowing is correct?

```rust
fn main() {
    let mut vec = vec![1, 2, 3];
    let first = &vec[0];
    vec.push(4);
    println!("{}", first);
}
```

- [ ] A) This code compiles because `first` is an immutable reference
- [ ] B) This code fails because you cannot have immutable and mutable references simultaneously
- [ ] C) This code fails because `vec.push(4)` requires a mutable reference while `first` holds an immutable reference
- [ ] D) This code compiles because the mutable borrow in `push` ends before using `first`

**Concept tested:** Borrowing rules and reference validity