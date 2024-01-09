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

    while let Some(event) = stream.next().await {
        let data = event?.data;
        let response: TensorswapOrderUpdateAllResponse =
            data.unwrap().tswap_order_update_all.unwrap();
        dbg!(&response);
    }

    Ok(())
}
