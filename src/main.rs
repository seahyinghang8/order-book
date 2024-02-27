use book::OrderBook;


mod linked_list;
mod price_tree;
mod order;
mod book;
mod clearing_house;


// TODO:
// 1. order_book + match algo
// 2. review
// 3. Server
// 4. Tests for linked_list to check the id returned
// 4. More tests (linked_list to check the id returned, btree edge cases)
fn main() {
    OrderBook::new();
}
