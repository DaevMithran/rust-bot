#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_book_creation() {
        let book = Book::new(
            "Test Book".to_string(),
            "Test Author".to_string(),
            "123456789".to_string(),
        );
        assert_eq!(book.title, "Test Book");
        assert_eq!(book.author, "Test Author");
        assert_eq!(book.isbn, "123456789");
    }

    #[test]
    fn test_member_creation() {
        let member = Member::new("Alice".to_string(), 1);
        assert_eq!(member.name, "Alice");
        assert_eq!(member.id, 1);
        assert_eq!(member.borrowed_books.len(), 0);
    }

    #[test]
    fn test_library_add_book() {
        let mut library = Library::new();
        let book = create_sample_book();
        
        library.add_book(book);
        assert_eq!(library.book_count(), 1);
        
        // This should not compile if book is used after move:
        // println!("{:?}", book); // Uncomment to test ownership
    }

    #[test]
    fn test_lending_and_returning() {
        let mut library = Library::new();
        let mut member = Member::new("Bob".to_string(), 2);
        
        let book = create_sample_book();
        let isbn = book.isbn.clone();
        library.add_book(book);
        
        // Lend book
        assert!(library.lend_book(&isbn, &mut member).is_ok());
        assert_eq!(library.book_count(), 0);
        assert_eq!(member.borrowed_books.len(), 1);
        assert!(member.has_book(&isbn));
        
        // Return book
        assert!(library.return_book(&isbn, &mut member).is_ok());
        assert_eq!(library.book_count(), 1);
        assert_eq!(member.borrowed_books.len(), 0);
        assert!(!member.has_book(&isbn));
    }

    #[test]
    fn test_lending_nonexistent_book() {
        let mut library = Library::new();
        let mut member = Member::new("Charlie".to_string(), 3);
        
        let result = library.lend_book("nonexistent", &mut member);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Book not found");
    }

    #[test]
    fn test_returning_book_not_borrowed() {
        let mut library = Library::new();
        let mut member = Member::new("Dave".to_string(), 4);
        
        let result = library.return_book("123456789", &mut member);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Member doesn't have this book");
    }

    #[test]
    fn test_search_by_title() {
        let mut library = Library::new();
        
        let book1 = Book::new("Rust Book".to_string(), "Author1".to_string(), "111".to_string());
        let book2 = Book::new("Python Book".to_string(), "Author2".to_string(), "222".to_string());
        let book3 = Book::new("Rust Book".to_string(), "Author3".to_string(), "333".to_string());
        
        library.add_book(book1);
        library.add_book(book2);
        library.add_book(book3);
        
        let rust_books = library.search_by_title("Rust Book");
        assert_eq!(rust_books.len(), 2);
        
        let python_books = library.search_by_title("Python Book");
        assert_eq!(python_books.len(), 1);
        assert_eq!(python_books[0].author, "Author2");
    }

    #[test]
    fn test_available_books_reference() {
        let mut library = Library::new();
        library.add_book(create_sample_book());
        
        let books_ref = library.available_books();
        assert_eq!(books_ref.len(), 1);
        
        // This should work - we're only borrowing
        let books_ref2 = library.available_books();
        assert_eq!(books_ref2.len(), 1);
        
        // Both references should be usable
        assert_eq!(books_ref.len(), books_ref2.len());
    }

    #[test]
    fn test_member_book_operations() {
        let mut member = Member::new("Eve".to_string(), 5);
        let book = create_sample_book();
        let isbn = book.isbn.clone();
        
        // Borrow book
        member.borrow_book(book);
        assert!(member.has_book(&isbn));
        assert_eq!(member.borrowed_books.len(), 1);
        
        // Return book
        let returned_book = member.return_book(&isbn);
        assert!(returned_book.is_some());
        assert!(!member.has_book(&isbn));
        assert_eq!(member.borrowed_books.len(), 0);
        
        let book = returned_book.unwrap();
        assert_eq!(book.isbn, isbn);
    }
}