#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_creation_and_book_addition() {
        let mut library = Library::new();
        let book_id = library.add_book(
            "The Rust Programming Language".to_string(),
            "Steve Klabnik".to_string(),
            "978-1718500440".to_string(),
        );
        
        assert_eq!(book_id, 1);
        assert!(library.get_book_info(book_id).is_some());
    }

    #[test]
    fn test_checkout_and_return_workflow() {
        let mut library = Library::new();
        let book_id = library.add_book(
            "Programming Rust".to_string(),
            "Jim Blandy".to_string(),
            "978-1492052590".to_string(),
        );

        // Book should be available initially
        assert!(library.get_book_info(book_id).is_some());

        // Check out the book
        let checked_out = library.checkout_book(
            book_id,
            "Alice".to_string(),
            "2024-02-01".to_string(),
        );
        assert!(checked_out.is_some());

        // Book should no longer be available in library
        assert!(library.get_book_info(book_id).is_none());

        // Return the book
        library.return_book(checked_out.unwrap());

        // Book should be available again
        assert!(library.get_book_info(book_id).is_some());
    }

    #[test]
    fn test_checkout_nonexistent_book() {
        let mut library = Library::new();
        let result = library.checkout_book(999, "Bob".to_string(), "2024-02-01".to_string());
        assert!(result.is_none());
    }

    #[test]
    fn test_checked_out_book_operations() {
        let mut library = Library::new();
        let book_id = library.add_book(
            "Rust in Action".to_string(),
            "Tim McNamara".to_string(),
            "978-1617294556".to_string(),
        );

        let mut checked_out = library.checkout_book(
            book_id,
            "Charlie".to_string(),
            "2024-02-01".to_string(),
        ).unwrap();

        // Test getting book info from checked out book
        let book_info = checked_out.get_book_info();
        assert_eq!(book_info.title, "Rust in Action");

        // Test extending due date
        checked_out.extend_due_date("2024-02-15".to_string());
        assert_eq!(checked_out.due_date, "2024-02-15");
    }

    #[test]
    fn test_find_books_by_title() {
        let mut library = Library::new();
        library.add_book("Rust Book 1".to_string(), "Author 1".to_string(), "ISBN1".to_string());
        library.add_book("Python Guide".to_string(), "Author 2".to_string(), "ISBN2".to_string());
        library.add_book("Rust Book 2".to_string(), "Author 3".to_string(), "ISBN3".to_string());

        let rust_books = library.find_books_by_title("Rust");
        assert_eq!(rust_books.len(), 2);

        let python_books = library.find_books_by_title("Python");
        assert_eq!(python_books.len(), 1);

        let no_books = library.find_books_by_title("Java");
        assert_eq!(no_books.len(), 0);
    }

    #[test]
    fn test_find_books_excludes_checked_out() {
        let mut library = Library::new();
        let id1 = library.add_book("Rust Book 1".to_string(), "Author 1".to_string(), "ISBN1".to_string());
        library.add_book("Rust Book 2".to_string(), "Author 2".to_string(), "ISBN2".to_string());

        // Check out one Rust book
        let _checked_out = library.checkout_book(id1, "Dave".to_string(), "2024-02-01".to_string());

        // Should only find the available Rust book
        let available_rust_books = library.find_books_by_title("Rust");
        assert_eq!(available_rust_books.len(), 1);
        assert_eq!(available_rust_books[0].title, "Rust Book 2");
    }

    #[test]
    fn test_get_available_books_iterator() {
        let mut library = Library::new();
        library.add_book("Book 1".to_string(), "Author 1".to_string(), "ISBN1".to_string());
        let id2 = library.add_book("Book 2".to_string(), "Author 2".to_string(), "ISBN2".to_string());
        library.add_book("Book 3".to_string(), "Author 3".to_string(), "ISBN3".to_string());

        // Check out one book
        let _checked_out = library.checkout_book(id2, "Eve".to_string(), "2024-02-01".to_string());

        // Should have 2 available books
        let available_books: Vec<&Book> = library.get_available_books().collect();
        assert_eq!(available_books.len(), 2);
        
        // Verify the checked out book is not in available books
        let titles: Vec<&String> = available_books.iter().map(|book| &book.title).collect();
        assert!(!titles.contains(&&"Book 2".to_string()));
    }

    #[test]
    fn test_multiple_checkouts_and_returns() {
        let mut library = Library::new();
        let id1 = library.add_book("Book 1".to_string(), "Author 1".to_string(), "ISBN1".to_string());
        let id2 = library.add_book("Book 2".to_string(), "Author 2".to_string(), "ISBN2".to_string());

        let checkout1 = library.checkout_book(id1, "User1".to_string(), "2024-02-01".to_string()).unwrap();
        let checkout2 = library.checkout_book(id2, "User2".to_string(), "2024-02-02".to_string()).unwrap();

        // No books should be available
        assert_eq!(library.get_available_books().count(), 0);

        // Return one book
        library.return_book(checkout1);
        assert_eq!(library.get_available_books().count(), 1);

        // Return second book
        library.return_book(checkout2);
        assert_eq!(library.get_available_books().count(), 2);
    }
}