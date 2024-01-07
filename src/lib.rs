use anyhow::Result;
use futures::SinkExt;

use tokio::sync::mpsc;
use tokio_stream::StreamExt;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{self, client::IntoClientRequest, http::header::HeaderValue},
};
use url::Url;

pub mod graphql;

use graphql::subscription::{Payload, SubscriptionClient};

const URL: &str = "wss://api.tensor.so/graphql";

/// Connect to a new WebSocket GraphQL server endpoint, and return a `SubscriptionClient`.
/// This method will:
/// a) connect to a ws(s):// endpoint, and perform the initial handshake, and
/// b) set up channel forwarding to expose just the returned `Payload`s to the client.
pub async fn connect_subscription_client() -> Result<SubscriptionClient, tungstenite::Error> {
    let url = Url::parse(URL).unwrap();

    let mut request = url.into_client_request().unwrap();
    request.headers_mut().insert(
        "Sec-WebSocket-Protocol",
        HeaderValue::from_str("graphql-transport-ws").unwrap(),
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
        while let Some(Ok(tungstenite::Message::Text(message))) = ws_rx.next().await {
            if let Ok(payload) = serde_json::from_str::<Payload>(&message) {
                _ = recv_tx.send(payload);
            }
        }
    });

    Ok(SubscriptionClient::new(send_tx, recv_rx))
}
