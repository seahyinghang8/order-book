use std::time::Instant;
use uuid::Uuid;

pub struct Order {
    id: Uuid,
    quantity: u32,
    price: u32,
    created_at: Instant,
}

impl Order {
    pub fn new(price: u32, quantity: u32) -> Order {
        Order {
            id: Uuid::new_v4(),
            price,
            quantity,
            created_at: Instant::now(),
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn price(&self) -> u32 {
        self.price
    }

    pub fn quantity(&self) -> u32 {
        self.quantity
    }

    pub fn update_quantity(&mut self, quantity: u32) {
        self.quantity = quantity
    }

    pub fn created_at(&self) -> Instant {
        self.created_at
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_new_order() {
        let order = Order::new(100, 5);
        assert_eq!(order.price(), 100);
        assert_eq!(order.quantity(), 5);
        // Since the created_at field is set to Instant::now(), we can't predict its exact value.
        // However, we can check that it's within a reasonable range of the current time.
        let now = Instant::now();
        assert!(order.created_at() >= now - Duration::from_secs(1));
        assert!(order.created_at() <= now + Duration::from_secs(1));
    }

    #[test]
    fn test_order_price() {
        let order = Order::new(200, 10);
        assert_eq!(order.price(), 200);
    }

    #[test]
    fn test_order_quantity() {
        let order = Order::new(300, 15);
        assert_eq!(order.quantity(), 15);
    }

    #[test]
    fn test_order_quantity_update() {
        let mut order = Order::new(300, 15);
        assert_eq!(order.quantity(), 15);
        order.update_quantity(10);
        assert_eq!(order.quantity(), 10);
    }

    #[test]
    fn test_order_created_at() {
        let order = Order::new(400, 20);
        let now = Instant::now();
        assert!(order.created_at() >= now - Duration::from_secs(1));
        assert!(order.created_at() <= now + Duration::from_secs(1));
    }
}
