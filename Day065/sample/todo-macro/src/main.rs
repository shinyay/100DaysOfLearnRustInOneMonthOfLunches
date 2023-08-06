struct Book {}

enum BookCategory {
    Mistery,
    Fantasy,
    Action,
    Technology,
}
 
fn get_book(book: &Book) -> Option<String> {} 

fn delete_book(book: &Book) -> Result<(), String> {} 

fn check_book_category(book_category: &BookCategory) {
    match book_category {
        BookCategory::Action => println!("The Category is Action"),
        BookCategory::Fantasy => println!("The Category is Fantasy"),
        BookCategory::Mistery => println!("The Category is Mistery"),
        BookCategory::Technology => println!("The Category is Technology"),
    }
}
 
fn main() {
    let book_type = BookType::HardCover;
    check_book_type(&book_type);
}
