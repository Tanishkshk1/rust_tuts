// In this lesson we will learn about ownership in rust
#[derive(Debug)]
struct Grocery {
    quantity: i32,
    id_no: i32,
}
fn display_quantity(rocery: &Grocery) {
    println!("quantity = {:?}", rocery.quantity);
}
fn display_id(rocery: &Grocery) {
    println!("id no = {:?}", rocery.id_no);
}
fn main() {
    let rocery = Grocery {
        id_no: 10,
        quantity: 14,
    };
    display_id(&rocery);
    display_quantity(&rocery);
}
//#[derive(Debug)]
//struct BOOK {
//    pages: i32,
//    ratings: i32,
//}
//fn display_page(book: &BOOK) {
//    // To make a function borrow something we provide the reference to
//    // the variable
//    println!("pages {:?}", book.pages);
//}
//fn display_ratings(book: &BOOK) {
//    println!("the ratings are {:?}", book.ratings);
//}
//fn main() {
//    let book = BOOK {
//        ratings: 3,
//        pages: 10,
//    };
//    display_page(&book); // In this way the function is just borrowing the dada rather than
//                         // transfering the ownership
//    display_ratings(&book);
//}
//enum Liht {
//    Briht,
//    Dull,
//}
//fn display_li(liht: &Liht) {
//    match liht {
//        Liht::Briht => println!("Briht"),
//        Liht::Dull => println!("dull"),
//    }
//}
//fn main() {
//    let dull = Liht::Dull;
//    let briht = Liht::Briht;
//    display_li(&dull);
//    display_li(&dull);
//    display_li(&briht);
//}
