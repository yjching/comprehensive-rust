// fn main() {
//     let mut vec = vec![10, 20];
//     vec.push(30);
//     let midpoint = vec.len() / 2;
//     println!("middle value: {}", vec[midpoint]);
//     for item in &vec {
//         println!("item: {item}");
//     }
// }

struct Library {
    books: Vec<Book>,
}

struct Book {
    title: String,
    year: u16,
}

impl Book {
    // This is a constructor, used below.
    fn new(title: &str, year: u16) -> Book {
        Book {
            title: String::from(title),
            year,
        }
    }
}

// Implement the methods below. Update the `self` parameter to
// indicate the method's required level of ownership over the object:
//
// - `&self` for shared read-only access,
// - `&mut self` for unique and mutable access,
// - `self` for unique access by value.
impl Library {
    fn new() -> Library {
        Library{ books: Vec::new() }
    }

    fn len(&self) -> usize {
       self.books.len()
    }

    fn is_empty(&self) -> bool {
       self.books.is_empty()
    }

    fn add_book(&mut self, book: Book) {
       self.books.push(book)
    }

    fn print_books(&self) {
        for book in self.books.iter() {
            println!("The book title is {}, published in {}", book.title, book.year)
        }
    }

    fn oldest_book(&self) -> Option<&Book> {
        let mut oldest: Option<&Book> = None;
        for book in self.books.iter() {
            if oldest.is_none() || book.year < oldest.unwrap().year {
                oldest = Some(book);
            }
        }
        return oldest
    }
}

// This shows the desired behavior. Uncomment the code below and
// implement the missing methods. You will need to update the
// method signatures, including the "self" parameter! You may
// also need to update the variable bindings within main.
fn main() {
    let mut library = Library::new();

    println!("The library is empty: library.is_empty() -> {}", library.is_empty());
    //
    library.add_book(Book::new("Lord of the Rings", 1954));
    library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));
    //
    
    println!("The library is no longer empty: library.is_empty() -> {}", library.is_empty());
    println!("The library has {} books", library.len());
    
    //
    //
    library.print_books();
    //
    match library.oldest_book() {
       Some(book) => println!("The oldest book is {}", book.title),
       None => println!("The library is empty!"),
    }
    //
    // library.print_books();
}