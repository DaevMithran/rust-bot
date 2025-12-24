#[derive(Debug, PartialEq)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub isbn: String,
}

impl Book {
    pub fn new(title: String, author: String, isbn: String) -> Self {
        // TODO: Implement constructor
        Book {
            title, author, isbn
        }
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
        Member {
            name, id, borrowed_books: Vec::new()
        }
    }
    
    pub fn borrow_book(&mut self, book: Book) {
        // Think about ownership transfer here
        self.borrowed_books.push(book)
    }
    
    pub fn return_book(&mut self, isbn: &str) -> Option<Book> {
        // Consider: how do you remove an item from Vec and return ownership?
        if let Some(index) = self.borrowed_books.iter().position(|b| b.isbn == isbn) {
            Some(self.borrowed_books.remove(index))
        } else {
            None
        }
    }
    
    pub fn has_book(&self, isbn: &str) -> bool {
        // Think about borrowing vs ownership here
        self.borrowed_books.iter().any(|b| b.isbn == isbn)
    }
}

#[derive(Debug)]
pub struct Library {
    books: Vec<Book>,
}

impl Library {
    pub fn new() -> Self {
        Library {
            books: Vec::new()
        }
    }
    
    pub fn add_book(&mut self, book: Book) {
        // TODO: Add book to library
        // What happens to ownership of the book parameter?
        self.books.push(book);
    }
    
    pub fn lend_book(&mut self, isbn: &str, member: &mut Member) -> Result<(), String> {
        // TODO: Find book by ISBN, remove it from library, and give it to member
        // This is complex - you need to:
        // 1. Find the book (borrowing)
        // 2. Remove it from the vector (taking ownership)
        // 3. Transfer ownership to member
        // Return Err("Book not found") if ISBN doesn't exist
        if let Some(index) = self.books.iter().position(|b| b.isbn == isbn) {
            let book = self.books.remove(index);
            member.borrow_book(book);
            return Ok(());
        } 

        Err("Book not found".to_string())
    }
    
    pub fn return_book(&mut self, isbn: &str, member: &mut Member) -> Result<(), String> {
        // TODO: Take book back from member and add to library
        // Return Err("Member doesn't have this book") if member doesn't have it
        if let Some(book) = member.return_book(isbn) {
            self.add_book(book);
            return Ok(());
        }

        Err("Member doesn't have this book".to_string())
    }
    
    pub fn search_by_title(&self, title: &str) -> Vec<&Book> {
        // TODO: Return references to books with matching title
        // Think carefully about lifetimes here
        self.books.iter().filter(|b| b.title == title ).collect()
    }
    
    pub fn available_books(&self) -> &Vec<Book> {
        // TODO: Return reference to all available books
        // What's the lifetime relationship?
        &self.books
    }
    
    pub fn book_count(&self) -> usize {
        // TODO: Return number of books currently in library
        self.books.len()
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