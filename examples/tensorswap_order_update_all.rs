use anyhow::Result;
use futures::StreamExt;
use tensor_trade_stream::{
    subscribe, TensorswapOrderUpdateAllQuery, TensorswapOrderUpdateAllVariables,
};

#[tokio::main]
async fn main() -> Result<()> {
    let (_client, mut stream) =
        subscribe::<TensorswapOrderUpdateAllQuery>(TensorswapOrderUpdateAllVariables {}).await?;

    while let Some(item) = stream.next().await {
        dbg!(Some(item));
    }

    Ok(())
}
