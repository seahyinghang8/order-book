use std::collections::{HashMap, HashSet};

use anyhow::{anyhow, Result};
use uuid::Uuid;

use crate::{
    order::Order,
    price_tree::{OrderKey, PriceTree},
};

pub struct OrderBook {
    bid_tree: PriceTree,
    ask_tree: PriceTree,
    order_id_map: HashMap<Uuid, (OrderType, OrderKey)>,
    order_removed_set: HashSet<Uuid>
}

pub enum OrderType {
    Bid,
    Ask,
}

struct PartialOrderMatch {
    order_key: OrderKey,
    remaining_quantity: u32,
}

struct MatchOutcome {
    remaining_quantity: u32,
    full_order: Vec<(Uuid, OrderKey)>,
    partial_order: Option<PartialOrderMatch>,
}

impl OrderBook {
    pub fn new() -> OrderBook {
        OrderBook {
            bid_tree: PriceTree::new(),
            ask_tree: PriceTree::new(),
            order_id_map: HashMap::new(),
            order_removed_set: HashSet::new(),
        }
    }

    pub fn place_order(
        &mut self,
        price: u32,
        quantity: u32,
        order_type: OrderType,
    ) -> Result<Uuid> {
        if quantity == 0 || price == 0 {
            return Err(anyhow!("Price or quantity should be bigger than 0"));
        }

        let mut order = Order::new(price, quantity);
        let match_outcome = Self::find_matching_orders(&self, &order, &order_type);

        // Send orders to clearing house and remove from book
        // TODO -> Send order to clearing house

        let tree_to_remove = match order_type {
            OrderType::Ask => &mut self.bid_tree,
            OrderType::Bid => &mut self.ask_tree,
        };

        for (filled_order_id, key) in match_outcome.full_order {
            tree_to_remove.remove_order(key).unwrap();
            self.order_id_map.remove(&filled_order_id);
            self.order_removed_set.insert(filled_order_id);
        }

        match match_outcome.partial_order {
            Some(partial_order) => {
                tree_to_remove
                    .update_order_quantity(
                        partial_order.order_key,
                        partial_order.remaining_quantity,
                    )
                    .unwrap();
            }
            None => {}
        };

        order.update_quantity(match_outcome.remaining_quantity);
        let order_id = order.id();

        // If incoming order is unfulfilled, it will be added to the book as a resting order
        if order.quantity() > 0 {
            let tree_to_add = match order_type {
                OrderType::Ask => &mut self.ask_tree,
                OrderType::Bid => &mut self.bid_tree
            };
            let order_key = tree_to_add.insert_order(order);
            self.order_id_map.insert(order_id, (order_type, order_key));
        } else {
            self.order_removed_set.insert(order_id);
        }

        Ok(order_id)
    }

    fn find_matching_orders(&self, incoming_order: &Order, order_type: &OrderType) -> MatchOutcome {
        let mut remaining_quantity = incoming_order.quantity();
        // Find as many existing orders that can match the incoming order
        let mut full_matching_order: Vec<(Uuid, OrderKey)> = Vec::new();
        let mut partial_matching_order: Option<PartialOrderMatch> = None;

        let mut tree_iter = match order_type {
            OrderType::Ask => self.bid_tree.iter(),
            OrderType::Bid => self.ask_tree.iter(),
        };

        loop {
            if let Some((price_node_id, price_node_iter)) = match order_type {
                OrderType::Ask => {
                    match tree_iter.next() {
                        // Continue if remaining order price >= asking price
                        Some((price_node_id, price_node)) => {
                            if price_node.price() >= incoming_order.price() {
                                Some((price_node_id, price_node.iter()))
                            } else {
                                None
                            }
                        }
                        None => None,
                    }
                }
                OrderType::Bid => {
                    match tree_iter.next_back() {
                        // Continue if remaining order price <= bidding price
                        Some((price_node_id, price_node)) => {
                            if price_node.price() <= incoming_order.price() {
                                Some((price_node_id, price_node.iter()))
                            } else {
                                None
                            }
                        }
                        None => None,
                    }
                }
            } {
                // Iterate through orders from oldest to newest
                for (linked_list_node_id, existing_order) in price_node_iter {
                    let order_key = OrderKey::new(price_node_id, linked_list_node_id);
                    if existing_order.quantity() <= remaining_quantity {
                        full_matching_order.push((existing_order.id(), order_key));
                        remaining_quantity -= existing_order.quantity();
                    } else {
                        partial_matching_order = Some(PartialOrderMatch {
                            order_key,
                            remaining_quantity: existing_order.quantity() - remaining_quantity,
                        });
                        remaining_quantity = 0;
                    }

                    // Incoming order is completely filled
                    if remaining_quantity == 0 {
                        return MatchOutcome {
                            remaining_quantity,
                            full_order: full_matching_order,
                            partial_order: partial_matching_order,
                        };
                    }
                }
            } else {
                break;
            }
        }

        MatchOutcome {
            remaining_quantity,
            full_order: full_matching_order,
            partial_order: partial_matching_order,
        }
    }

    fn add_order_to_tree(&self, order_type: &OrderType, order: Order) {
        let order_id = order.id();
        let tree_to_add = match order_type {
                OrderType::Ask => &mut self.ask_tree,
                OrderType::Bid => &mut self.bid_tree
            };
            let order_key = tree_to_add.insert_order(order);

        self.order_id_map.insert(order_id, (order_type, order_key));
    }

    pub fn cancel_order(&self, order_id: Uuid) -> Result<()> {
        if self.order_removed_set.contains(&order_id) {
            return Err(anyhow!("Order is already removed from the book"));
        }

        if let Some(order_key) = self.order_id_map.get(&order_id) {
            let tree_to_remove = match order_type {
                OrderType::Ask => &mut self.ask_tree,
                OrderType::Bid => &mut self.bid_tree
            };
        }

        Err(anyhow!("Order annot be found"))
    }

    pub fn view_book_l2(&self) {
        todo!()
    }
}
