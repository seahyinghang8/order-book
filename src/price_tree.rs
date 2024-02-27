use slab::Slab;
use std::collections::BTreeMap;
use anyhow::{anyhow, Ok, Result};

use crate::{
    linked_list::{SlabLinkedList, SlabLinkedListIter},
    order::Order,
};

pub struct PriceNode {
    linked_list: SlabLinkedList<Order>,
    price: u32,
    total_quantity: u32,
}

impl PriceNode {
    pub fn price(&self) -> u32 {
        self.price
    }

    pub fn total_quantity(&self) -> u32 {
        self.total_quantity
    }

    pub fn iter(&self) -> PriceNodeIterator {
        PriceNodeIterator {
            linked_list_iter: self.linked_list.iter(),
        }
    }
}

pub struct PriceNodeIterator<'a> {
    linked_list_iter: SlabLinkedListIter<'a, Order>,
}

impl<'a> Iterator for PriceNodeIterator<'a> {
    type Item = (usize, &'a Order);

    fn next(&mut self) -> Option<Self::Item> {
        self.linked_list_iter.next()
    }
}

pub struct OrderKey {
    price_node_id: usize,
    linked_list_node_id: usize,
}

impl OrderKey {
    pub fn new(price_node_id: usize, linked_list_node_id: usize) -> OrderKey {
        OrderKey {
            price_node_id,
            linked_list_node_id,
        }
    }
}

pub struct PriceTree {
    tree: BTreeMap<u32, usize>,
    slab: Slab<PriceNode>,
}

impl PriceTree {
    pub fn new() -> PriceTree {
        PriceTree {
            tree: BTreeMap::new(),
            slab: Slab::new(),
        }
    }

    pub fn insert_order(&mut self, order: Order) -> OrderKey {
        let price = order.price();
        match self.tree.get(&price) {
            Some(&price_node_id) => {
                // Get and insert order to price node's linked list
                let price_node = &mut self.slab[price_node_id];
                price_node.total_quantity += order.quantity();
                let linked_list_node_id = price_node.linked_list.push_back(order);

                OrderKey {
                    price_node_id,
                    linked_list_node_id,
                }
            }
            None => {
                // Create a price node
                let mut price_node = PriceNode {
                    linked_list: SlabLinkedList::new(),
                    price,
                    total_quantity: order.quantity(),
                };
                let linked_list_node_id = price_node.linked_list.push_back(order);
                let price_node_id = self.slab.insert(price_node);
                self.tree.insert(price, price_node_id);

                OrderKey {
                    price_node_id,
                    linked_list_node_id,
                }
            }
        }
    }

    pub fn remove_order(&mut self, key: &OrderKey) -> Result<()> {
        match self.slab.get_mut(key.price_node_id) {
            Some(price_node) => {
                match price_node.linked_list.remove(key.linked_list_node_id) {
                    Some(order) => {
                        if price_node.linked_list.is_empty() {
                            // Remove price node from slab and tree
                            self.slab.remove(key.price_node_id);
                            self.tree.remove(&order.price());
                        } else {
                            price_node.total_quantity -= order.quantity()
                        }
                        Ok(())
                    }
                    None => Err(anyhow!("Order does not exist in linked list")),
                }
            }
            None => Err(anyhow!("Order does not exist in tree")),
        }
    }

    // TODO: Needs testing
    pub fn update_order_quantity(&mut self, key: OrderKey, quantity: u32) -> Result<()> {
        match self.slab.get_mut(key.price_node_id) {
            Some(price_node) => {
                match price_node.linked_list.get_mut(key.linked_list_node_id) {
                    Some(order) => {
                        order.update_quantity(quantity);
                        Ok(())
                    }
                    None => Err(anyhow!("Order does not exist in linked list")),
                }
            }
            None => Err(anyhow!("Order does not exist in tree")),
        }
    }

    pub fn iter(&self) -> PriceTreeIterator {
        PriceTreeIterator {
            slab: &self.slab,
            tree_iter: self.tree.iter(),
        }
    }
}

pub struct PriceTreeIterator<'a, 'b> {
    slab: &'a Slab<PriceNode>,
    tree_iter: std::collections::btree_map::Iter<'b, u32, usize>,
}

impl<'a, 'b> Iterator for PriceTreeIterator<'a, 'b> {
    type Item = (usize, &'a PriceNode);

    fn next(&mut self) -> Option<Self::Item> {
        match self.tree_iter.next() {
            Some((_, &node_id)) => Some((node_id, &self.slab[node_id])),
            None => None,
        }
    }
}

// Reverse iterator
impl<'a, 'b> DoubleEndedIterator for PriceTreeIterator<'a, 'b> {
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.tree_iter.next_back() {
            Some((_, &node_id)) => Some((node_id, &self.slab[node_id])),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use rand::{self, Rng};

    #[test]
    fn test_insert_order_simple() {
        let mut price_tree = PriceTree::new();
        let order1 = Order::new(100, 10);
        let order2 = Order::new(150, 5);
        let order3 = Order::new(100, 4);

        let key1 = price_tree.insert_order(order1);
        let key2 = price_tree.insert_order(order2);
        price_tree.insert_order(order3);

        assert_eq!(price_tree.slab.len(), 2);
        assert_eq!(price_tree.slab[key1.price_node_id].total_quantity, 14);
        assert_eq!(price_tree.slab[key2.price_node_id].total_quantity, 5);
        assert_eq!(
            price_tree.slab[key1.price_node_id]
                .linked_list
                .front()
                .unwrap()
                .quantity(),
            10
        );
    }

    #[test]
    fn test_insert_order_advanced() {
        let mut rng = rand::thread_rng();
        let mut price_tree = PriceTree::new();

        let mut quantity_map: HashMap<u32, u32> = HashMap::new();

        for _ in 0..10000 {
            let price: u32 = rng.gen_range(1..30);
            let quantity: u32 = rng.gen_range(1..500);
            let order = Order::new(price, quantity);
            price_tree.insert_order(order);

            match quantity_map.get_mut(&price) {
                Some(total_quantity) => *total_quantity += quantity,
                None => {
                    quantity_map.insert(price, quantity);
                }
            }
        }

        for (price, &total_quantity) in quantity_map.iter() {
            assert_eq!(
                price_tree.slab[price_tree.tree[price]].total_quantity,
                total_quantity
            );
        }
    }

    #[test]
    fn test_remove_order() {
        let mut price_tree = PriceTree::new();
        let order = Order::new(200, 8);
        let key = price_tree.insert_order(order);

        assert_eq!(price_tree.remove_order(&key).unwrap(), ());
        assert_eq!(price_tree.slab.len(), 0);
    }

    #[test]
    fn test_iteration() {
        let mut price_tree = PriceTree::new();
        let order1 = Order::new(120, 3);
        let order2 = Order::new(140, 6);
        let order3 = Order::new(140, 2);

        price_tree.insert_order(order1);
        price_tree.insert_order(order2);
        price_tree.insert_order(order3);

        let mut iter = price_tree.iter();

        assert_eq!(iter.next().unwrap().1.total_quantity, 3);
        assert_eq!(iter.next().unwrap().1.total_quantity, 8);
    }

    #[test]
    fn test_iteration_rev() {
        let mut price_tree = PriceTree::new();
        let order1 = Order::new(120, 3);
        let order2 = Order::new(140, 6);
        let order3 = Order::new(140, 2);

        price_tree.insert_order(order1);
        price_tree.insert_order(order2);
        price_tree.insert_order(order3);

        let mut iter = price_tree.iter().rev();

        assert_eq!(iter.next().unwrap().1.total_quantity, 8);
        assert_eq!(iter.next().unwrap().1.total_quantity, 3);
    }
}
