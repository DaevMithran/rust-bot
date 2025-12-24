# Problem: Ownership & Borrowing

**Difficulty:** intermediate
**Time:** 30-45 minutes

---

You are tasked with implementing a simple library management system that demonstrates Rust's ownership and borrowing concepts. The system should handle books, library members, and book lending operations while strictly adhering to Rust's ownership rules.

**Requirements:**

1. **Book struct**: Contains title, author, and ISBN. Books can be moved between different states (available, borrowed).

2. **Member struct**: Represents a library member with a name and ID who can borrow books.

3. **Library struct**: Manages the collection of books and lending operations.

4. **Key functionality to implement**:
   - Add books to the library (transferring ownership)
   - Lend books to members (moving ownership from library to member)
   - Return books from members (moving ownership back to library)
   - Search for books by title (using borrowing, not moving)
   - List all available books (using borrowing)

**Ownership challenges you must solve**:
- Books should be moved (not copied) when lent/returned
- Search operations should not take ownership of the library
- Multiple borrows should be handled correctly
- Prevent use-after-move errors when books are lent out
- Handle lifetime parameters where necessary

**Constraints**:
- No `Clone` trait usage for Book struct
- Use `Vec<Book>` for storage, not references
- Implement proper error handling for books not found
- Use string slices (`&str`) for search parameters to avoid unnecessary ownership transfer

## Files
- `main.rs` - Implement your solution here
- `tests.rs` - Run with `cargo test`
