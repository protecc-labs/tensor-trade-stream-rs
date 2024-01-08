# tensor-trade-stream

A client for receiving updates from [Tensor Trade](https://www.tensor.trade/) NFT marketplace over WebSocket using GraphQL subscriptions via [`graphql_ws_client`](https://crates.io/crates/graphql-ws-client).

## Documentation

[Tensor Trade Documentation](https://tensor-hq.notion.site/PUBLIC-Tensor-Trade-API-Docs-alpha-b18e1a196187473bac9b5d6de5b47032#23a79268ff6e46bcb2d7d176eb2066da)

## Example

```rust
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

```
