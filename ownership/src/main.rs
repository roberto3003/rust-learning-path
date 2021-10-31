enum Light {
    Bright,
    Dull,
}

fn display_light(ligt: &Light) {
    match ligt {
        Light::Bright => println!("bright"),
        Light::Dull => println!("dull"),
    }
}

struct Book {
    pages: i32,
    rating: i32,
}

fn display_page_count(book: &Book) {
    println!("pages = {:?}", book.pages);
}

fn display_rating(book: Book) {
    println!("rating = {:?}", book.rating);
}

struct GroceryItem {
    quantity: i32,
    id: i32,
}

fn display_quantity(item: &GroceryItem) {
    println!("quantity = {:?}", item.quantity);
}

fn display_id(item: &GroceryItem) {
    println!("id = {:?}", item.id);
}

fn main() {
    let dull = Light::Dull;
    display_light(&dull);
    display_light(&dull);

    let book = Book {
        pages: 5,
        rating: 9,
    };
    display_page_count(&book);
    display_rating(book);

    let my_item = GroceryItem {
        quantity: 100,
        id: 1,
    };

    display_quantity(&my_item);
    display_id(&my_item);
}
