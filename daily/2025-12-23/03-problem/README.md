# Problem: Ownership & Borrowing

**Difficulty:** intermediate
**Time:** 30-45 minutes

---

You are building a library management system that tracks books and their borrowing history. The system needs to handle ownership transfers when books are checked out, borrowed references for reading metadata, and proper lifetime management for temporary operations.

Your task is to implement a `Library` struct that manages `Book` instances with the following requirements:

1. **Book Management**: Books can be added to the library and retrieved by ID
2. **Checkout System**: When a book is checked out, ownership transfers to the borrower (represented by a `CheckedOutBook` struct)
3. **Return System**: Checked out books can be returned to the library, transferring ownership back
4. **Metadata Access**: Provide borrowing-based access to book information without transferring ownership
5. **Search Functionality**: Find books by title using string slices with proper lifetime management

Key constraints:
- Books should be moved (not cloned) when checked out
- The system should prevent accessing checked-out books in the library
- All borrowing must respect Rust's ownership rules
- Use lifetimes appropriately for references that outlive function calls

## Files
- `main.rs` - Implement your solution here
- `tests.rs` - Run with `cargo test`
