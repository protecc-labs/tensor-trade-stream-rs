use async_tungstenite::tungstenite::{
    client::IntoClientRequest,
    http::{header, HeaderValue},
    Message,
};
use eyre::Result;
use futures::StreamExt;
use graphql_client::GraphQLQuery;
use graphql_ws_client::{
    graphql::{GraphQLClient, StreamingOperation},
    AsyncWebsocketClient, GraphQLClientClientBuilder, SubscriptionStream,
};

pub mod queries;
mod tokio_spawner;

use tokio_spawner::TokioSpawner;

pub type TransactionQuery = queries::NewTransactionTV2;
pub type TensorswapOrderUpdateQuery = queries::TswapOrderUpdate;
pub type TensorswapOrderUpdateAllQuery = queries::TswapOrderUpdateAll;

pub type TransactionVariables = queries::new_transaction_tv2::Variables;
pub type TensorswapOrderUpdateVariables = queries::tswap_order_update::Variables;
pub type TensorswapOrderUpdateAllVariables = queries::tswap_order_update_all::Variables;

pub type TransactionResponse = queries::new_transaction_tv2::NewTransactionTv2NewTransactionTv2;
pub type TensorswapOrderUpdateResponse =
    queries::tswap_order_update::TswapOrderUpdateTswapOrderUpdate;
pub type TensorswapOrderUpdateAllResponse =
    queries::tswap_order_update_all::TswapOrderUpdateAllTswapOrderUpdateAll;

pub async fn subscribe<T: GraphQLQuery + Send + Sync + Unpin + 'static>(
    api_key: &str,
    variables: T::Variables,
) -> Result<(
    AsyncWebsocketClient<GraphQLClient, Message>,
    SubscriptionStream<GraphQLClient, StreamingOperation<T>>,
)>
where
    <T as GraphQLQuery>::Variables: Send + Sync + Unpin,
    <T as GraphQLQuery>::ResponseData: std::fmt::Debug,
{
    let mut request = "wss://api.tensor.so/graphql".into_client_request()?;
    request.headers_mut().insert(
        header::SEC_WEBSOCKET_PROTOCOL,
        HeaderValue::from_str("graphql-transport-ws")?,
    );
    request
        .headers_mut()
        .insert("X-TENSOR-API-KEY", HeaderValue::from_str(api_key)?);

    let (connection, _) = async_tungstenite::tokio::connect_async(request).await?;

    let (sink, stream) = connection.split::<Message>();

    let mut client = GraphQLClientClientBuilder::new()
        .build(stream, sink, TokioSpawner::current())
        .await?;

    let stream = client
        .streaming_operation(StreamingOperation::<T>::new(variables))
        .await?;

    Ok((client, stream))
}
