use anyhow::Result;
use futures::SinkExt;

use graphql_client::GraphQLQuery;
use tokio::sync::mpsc;
use tokio_stream::StreamExt;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{
        self,
        client::IntoClientRequest,
        http::header::{self, HeaderValue},
    },
};
use url::Url;

pub mod graphql;

use graphql::{
    queries::{tswap_order_update, tswap_order_update_all, TswapOrderUpdate, TswapOrderUpdateAll},
    subscription::{BoxedSubscription, Payload, SubscriptionClient},
};

const URL: &str = "wss://api.tensor.so/graphql";

/// Connect to a new WebSocket GraphQL server endpoint, and return a `SubscriptionClient`.
/// This method will:
/// a) connect to a ws(s):// endpoint, perform the initial handshake, and
/// b) set up channel forwarding to expose just the returned `Payload`s to the client.
pub async fn connect_subscription_client() -> Result<SubscriptionClient, tungstenite::Error> {
    let url = Url::parse(URL).unwrap(); // TODO: This error is not handled.

    let mut request = url.into_client_request().unwrap(); // TODO: This error is not handled.
    request.headers_mut().insert(
        header::SEC_WEBSOCKET_PROTOCOL,
        HeaderValue::from_str("graphql-transport-ws").unwrap(), // TODO: This error is not handled.
    );

    let (ws, _) = connect_async(request).await?;

    let (mut ws_tx, mut ws_rx) = futures::StreamExt::split(ws);

    let (send_tx, mut send_rx) = mpsc::unbounded_channel::<Payload>();
    let (recv_tx, recv_rx) = mpsc::unbounded_channel::<Payload>();

    // Forwarded received messages back upstream to the GraphQL server.
    tokio::spawn(async move {
        while let Some(payload) = send_rx.recv().await {
            _ = ws_tx
                .send(tungstenite::Message::Text(
                    serde_json::to_string(&payload).unwrap(),
                ))
                .await;
        }
    });

    // Forward received messages to the receiver channel.
    tokio::spawn(async move {
        dbg!(&ws_rx);
        while let Some(Ok(tungstenite::Message::Text(message))) = ws_rx.next().await {
            if let Ok(payload) = serde_json::from_str::<Payload>(&message) {
                _ = recv_tx.send(payload);
            }
        }
    });

    Ok(SubscriptionClient::new(send_tx, recv_rx))
}

pub fn subscribe_all(
    subscription_client: &SubscriptionClient,
) -> BoxedSubscription<TswapOrderUpdateAll> {
    let request_body = TswapOrderUpdateAll::build_query(tswap_order_update_all::Variables {});

    subscription_client.start::<TswapOrderUpdateAll>(&request_body)
}

pub fn subscribe_collection(
    subscription_client: &SubscriptionClient,
    slug: &str,
) -> BoxedSubscription<TswapOrderUpdate> {
    let request_body = TswapOrderUpdate::build_query(tswap_order_update::Variables {
        slug: slug.to_string(),
    });

    subscription_client.start::<TswapOrderUpdate>(&request_body)
}
