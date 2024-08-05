use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
struct OrderBookUpdate {
    e: String, // Event type
    E: u64,    // Event time
    s: String, // Symbol
    U: u64,    // First update ID in event
    u: u64,    // Final update ID in event
    b: Vec<[String; 2]>, // Bids to be updated
    a: Vec<[String; 2]>, // Asks to be updated
}

struct LocalOrderBook {
    symbol: String,
    bids: BTreeMap<f64, f64>,
    asks: BTreeMap<f64, f64>,
    last_update_id: u64,
}

impl LocalOrderBook {
    fn new(symbol: String) -> Self {
        LocalOrderBook {
            symbol,
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            last_update_id: 0,
        }
    }

    fn update(&mut self, update: OrderBookUpdate) {
        if update.u <= self.last_update_id {
            return;
        }

        self.last_update_id = update.u;

        // Update bids
        for bid in update.b {
            let price = bid[0].parse::<f64>().unwrap();
            let quantity = bid[1].parse::<f64>().unwrap();
            if quantity == 0.0 {
                self.bids.remove(&price);
            } else {
                self.bids.insert(price, quantity);
            }
        }

        // Update asks
        for ask in update.a {
            let price = ask[0].parse::<f64>().unwrap();
            let quantity = ask[1].parse::<f64>().unwrap();
            if quantity == 0.0 {
                self.asks.remove(&price);
            } else {
                self.asks.insert(price, quantity);
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let symbol = "btcusdt".to_lowercase();
    let mut local_order_book = LocalOrderBook::new(symbol.clone());

    let url = Url::parse(&format!(
        "wss://stream.binance.com:9443/ws/{}@depth",
        symbol
    ))?;

    let (ws_stream, _) = connect_async(url).await?;
    println!("WebSocket connected");

    let (_, read) = ws_stream.split();

    read.for_each(|message| async {
        match message {
            Ok(Message::Text(text)) => {
                let v: Value = serde_json::from_str(&text).unwrap();
                if let Ok(update) = serde_json::from_value::<OrderBookUpdate>(v) {
                    local_order_book.update(update);
                    println!("Updated local order book for {}", symbol);
                    println!("Top bid: {:?}", local_order_book.bids.iter().next_back());
                    println!("Top ask: {:?}", local_order_book.asks.iter().next());
                }
            }
            Ok(Message::Close(..)) => println!("WebSocket closed"),
            Err(e) => eprintln!("Error processing message: {}", e),
            _ => {}
        }
    })
    .await;

    Ok(())
}