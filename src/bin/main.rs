use tokio::time;
use tokio_stream::StreamExt;

extern crate tensor_trade_stream;

use tensor_trade_stream::{connect_subscription_client, subscribe_collection};

const RECONNECT_DELAY: u64 = 500;

#[tokio::main]
async fn main() {
    loop {
        let subscription_client = match connect_subscription_client().await {
            Ok(c) => c,
            Err(_) => {
                time::sleep(time::Duration::from_millis(RECONNECT_DELAY)).await;
                continue;
            }
        };

        let mut stream = subscribe_collection(&subscription_client, "duckpunkzuniverse");

        println!("streaming...");

        while let Some(message) = stream.next().await {
            dbg!(&message);

            if let Some(response) = message {
                if let Some(response_data) = response.data {
                    if let Some(tswap_order_update) = response_data.tswap_order_update {
                        let pool = &tswap_order_update.pool;

                        if pool.is_none() {
                            println!("pool is none");
                        } else {
                            dbg!(tswap_order_update);
                        }
                    }
                }
            }
        }

        println!("stream ended");
    }
}
