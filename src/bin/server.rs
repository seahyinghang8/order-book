use anyhow::Result;
use std::sync::Arc;
use tokio::{
    net::{TcpListener, TcpStream},
    sync::RwLock,
};

use order_book::{resp::Response, req::Request, book::OrderBook, wire::{read_msg, write_msg}};


async fn process_socket(mut socket: TcpStream, book: Arc<RwLock<OrderBook>>) {
    loop {
        // Deserialize incoming request
        match read_msg(&mut socket).await {
            Ok(msg) => {
                match msg {
                    Request::ViewL2Book => {
                        let book = book.read().await;
                        let l2_book = book.view_book_l2();
                        write_msg(&mut socket, &Response::L2BookOk(l2_book))
                            .await
                            .unwrap();
                    }
                    Request::CancelOrder(orders_args) => {
                        let mut book = book.write().await;
                        match book.cancel_order(orders_args.order_id) {
                            Ok(()) => write_msg(&mut socket, &Response::CancelOk).await.unwrap(),
                            Err(_) => write_msg(&mut socket, &Response::CancelErr).await.unwrap(),
                        }
                    }
                    Request::PlaceOrder(place_order_args) => {
                        let mut book = book.write().await;
                        match book.place_order(
                            place_order_args.price,
                            place_order_args.quantity,
                            place_order_args.order_type,
                        ) {
                            Ok(order_id) => write_msg(&mut socket, &Response::PlaceOk(order_id))
                                .await
                                .unwrap(),
                            Err(_) => write_msg(&mut socket, &Response::PlacErr).await.unwrap(),
                        }
                    }
                }
                // Write response
            }
            Err(_) => {
                // Failed to parse request
                return;
            }
        };
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let book = Arc::new(RwLock::new(OrderBook::new()));
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (socket, _) = listener.accept().await?;
        let book = book.clone();

        tokio::spawn(async move {
            process_socket(socket, book).await;
        });
    }
}
