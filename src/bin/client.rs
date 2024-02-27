use anyhow::Result;
use tokio::net::TcpStream;

use order_book::{
    book::OrderType,
    req::{CancelOrderArgs, PlaceOrderArgs, Request},
    resp::Response,
    wire::{read_msg, write_msg},
};

use clap::{Parser, Subcommand};
use uuid::Uuid;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    PlaceOrder {
        #[clap(long, short, action)]
        is_bid: bool,
        price: u32,
        quantity: u32,
    },
    CancelOrder {
        order_id: Uuid,
    },
    ViewL2Book,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::PlaceOrder {
            is_bid,
            price,
            quantity,
        }) => {
            let order_type = if *is_bid {
                OrderType::Bid
            } else {
                OrderType::Ask
            };
            process_request(Request::PlaceOrder(PlaceOrderArgs { order_type, quantity: *quantity, price: *price })).await.unwrap();
        }
        Some(Commands::CancelOrder { order_id }) => {
            process_request(Request::CancelOrder(CancelOrderArgs { order_id: *order_id })).await.unwrap();
        }
        Some(Commands::ViewL2Book) => {
            process_request(Request::ViewL2Book).await.unwrap();
        }
        None => {
            println!("No command issued");
        }
    }

    Ok(())
}

async fn process_request(request: Request) -> Result<()> {
    let mut socket = TcpStream::connect("127.0.0.1:8080").await?;
    write_msg(&mut socket, &request).await.unwrap();
    let response: Response = read_msg(&mut socket).await.unwrap();
    println!("Response: {:?}", response);
    Ok(())
}
