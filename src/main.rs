use book::OrderBook;

mod book;
mod linked_list;
mod order;
mod price_tree;

// TODO:
// 3. Server
// 4. Tests for linked_list to check the id returned
// 4. More tests (linked_list to check the id returned, btree edge cases)
fn main() {
    let book = OrderBook::new();
}
