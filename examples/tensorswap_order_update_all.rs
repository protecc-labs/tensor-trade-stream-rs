use eyre::Result;
use futures::StreamExt;
use tensor_trade_stream::{
    connect, subscribe, TensorswapOrderUpdateAllQuery, TensorswapOrderUpdateAllResponse,
    TensorswapOrderUpdateAllVariables,
};

#[tokio::main]
async fn main() -> Result<()> {
    let client = match connect().await {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Error connecting to websocket: {}", e);
            return Ok(());
        }
    };

    let mut stream =
        subscribe::<TensorswapOrderUpdateAllQuery>(client, TensorswapOrderUpdateAllVariables {})
            .await?;

    while let None = stream.next().await {
        // let data = event?.data;
        // let response: TensorswapOrderUpdateAllResponse =
        //     data.unwrap().tswap_order_update_all.unwrap();
        dbg!(&stream.next().await);
    }

    Ok(())
}
