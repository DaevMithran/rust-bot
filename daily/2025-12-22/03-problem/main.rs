#[derive(Debug, PartialEq)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub isbn: String,
}

impl Book {
    pub fn new(title: String, author: String, isbn: String) -> Self {
        // TODO: Implement constructor
    }
}

#[derive(Debug)]
pub struct Member {
    pub name: String,
    pub id: u32,
    pub borrowed_books: Vec<Book>,
}

impl Member {
    pub fn new(name: String, id: u32) -> Self {
        // TODO: Implement constructor
    }
    
    pub fn borrow_book(&mut self, book: Book) {
        // TODO: Add book to member's borrowed books
        // Think about ownership transfer here
    }
    
    pub fn return_book(&mut self, isbn: &str) -> Option<Book> {
        // TODO: Remove and return book with matching ISBN
        // Consider: how do you remove an item from Vec and return ownership?
    }
    
    pub fn has_book(&self, isbn: &str) -> bool {
        // TODO: Check if member has book with given ISBN
        // Think about borrowing vs ownership here
    }
}

#[derive(Debug)]
pub struct Library {
    books: Vec<Book>,
}

impl Library {
    pub fn new() -> Self {
        // TODO: Create empty library
    }
    
    pub fn add_book(&mut self, book: Book) {
        // TODO: Add book to library
        // What happens to ownership of the book parameter?
    }
    
    pub fn lend_book(&mut self, isbn: &str, member: &mut Member) -> Result<(), String> {
        // TODO: Find book by ISBN, remove it from library, and give it to member
        // This is complex - you need to:
        // 1. Find the book (borrowing)
        // 2. Remove it from the vector (taking ownership)
        // 3. Transfer ownership to member
        // Return Err("Book not found") if ISBN doesn't exist
    }
    
    pub fn return_book(&mut self, isbn: &str, member: &mut Member) -> Result<(), String> {
        // TODO: Take book back from member and add to library
        // Return Err("Member doesn't have this book") if member doesn't have it
    }
    
    pub fn search_by_title(&self, title: &str) -> Vec<&Book> {
        // TODO: Return references to books with matching title
        // Think carefully about lifetimes here
    }
    
    pub fn available_books(&self) -> &Vec<Book> {
        // TODO: Return reference to all available books
        // What's the lifetime relationship?
    }
    
    pub fn book_count(&self) -> usize {
        // TODO: Return number of books currently in library
    }
}

// Helper function for tests
pub fn create_sample_book() -> Book {
    Book::new(
        "The Rust Programming Language".to_string(),
        "Steve Klabnik".to_string(),
        "978-1718500440".to_string(),
    )
}