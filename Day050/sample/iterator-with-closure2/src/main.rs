struct Library {
    name: String,
    category: Option<String>,
}

impl Library {
    fn new(name: &str, category: &str) -> Self {
        let category = match category {
            "" => None,
            category => Some(category.to_string()),
        };
        Self {
            name: name.to_string(),
            category
        }
    }

    fn get_category(&self) -> Option<String> {
        self.category.clone()
    }
}

fn main() {
    let books_vec = vec![
        Library::new("Rust in Action", "Rust"),
        Library::new("Spring in Action", "Spring"),
        Library::new("Kotlin in Practice", "Kotlin"),
        Library::new("WebAssemly Getting Started", "WebAssembly"),
        Library::new("vSphere for Beginners", ""),
        Library::new("English Grammer for Dummy", ""),
    ];

    let mut my_vec = vec![];

    books_vec.iter()
        .for_each(|book| my_vec.push(book.get_category()
        .ok_or_else(|| {
            let error_msg = format!("Category is Not Found for {}", book.name);
            error_msg
        })));

    for result in my_vec {
        println!("{result:?}");
    }
}