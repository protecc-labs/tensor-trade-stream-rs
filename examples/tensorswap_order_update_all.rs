use eyre::Result;
use futures::StreamExt;
use tensor_trade_stream::{
    subscribe, TensorswapOrderUpdateAllQuery, TensorswapOrderUpdateAllVariables,
};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let (_client, mut stream) = subscribe::<TensorswapOrderUpdateAllQuery>(
        &std::env::var("TENSOR_TRADE_API_KEY")?,
        TensorswapOrderUpdateAllVariables {},
    )
    .await?;

    while let Some(response) = stream.next().await {
        dbg!(&response);
    }

    Ok(())
}
