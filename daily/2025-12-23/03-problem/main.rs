#[derive(Debug, Clone, PartialEq)]
pub struct Book {
    pub id: u32,
    pub title: String,
    pub author: String,
    pub isbn: String,
}

#[derive(Debug)]
pub struct CheckedOutBook {
    pub book: Book,
    pub borrower_name: String,
    pub due_date: String,
}

#[derive(Debug)]
pub struct Library {
    books: std::collections::HashMap<u32, Book>,
    next_id: u32,
}

impl Library {
    pub fn new() -> Self {
        // TODO: Initialize a new empty library
        todo!()
    }

    pub fn add_book(&mut self, title: String, author: String, isbn: String) -> u32 {
        // TODO: Add a new book to the library and return its ID
        // The book should be assigned the next available ID
        todo!()
    }

    pub fn checkout_book(&mut self, book_id: u32, borrower_name: String, due_date: String) -> Option<CheckedOutBook> {
        // TODO: Remove the book from the library and return it wrapped in CheckedOutBook
        // Return None if the book doesn't exist
        // This should transfer ownership of the book
        todo!()
    }

    pub fn return_book(&mut self, checked_out_book: CheckedOutBook) {
        // TODO: Take ownership of the CheckedOutBook and return the book to the library
        todo!()
    }

    pub fn get_book_info(&self, book_id: u32) -> Option<&Book> {
        // TODO: Return a reference to the book without transferring ownership
        // Return None if book doesn't exist or is checked out
        todo!()
    }

    pub fn find_books_by_title(&self, title_query: &str) -> Vec<&Book> {
        // TODO: Return references to all books whose titles contain the query string
        // Only include books that are currently in the library (not checked out)
        todo!()
    }

    pub fn get_available_books(&self) -> impl Iterator<Item = &Book> {
        // TODO: Return an iterator over references to all available books
        todo!()
    }
}

impl CheckedOutBook {
    pub fn get_book_info(&self) -> &Book {
        // TODO: Return a reference to the book inside the CheckedOutBook
        todo!()
    }

    pub fn extend_due_date(&mut self, new_due_date: String) {
        // TODO: Update the due date for this checked out book
        todo!()
    }
}