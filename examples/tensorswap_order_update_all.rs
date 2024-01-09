use eyre::Result;
use futures::StreamExt;
use tensor_trade_stream::{
    subscribe, TensorswapOrderUpdateAllQuery, TensorswapOrderUpdateAllResponse,
    TensorswapOrderUpdateAllVariables,
};

#[tokio::main]
async fn main() -> Result<()> {
    let (_client, mut stream) =
        subscribe::<TensorswapOrderUpdateAllQuery>(TensorswapOrderUpdateAllVariables {}).await?;

    while let Some(response) = stream.next().await {
        dbg!(&response);
    }

    Ok(())
}
