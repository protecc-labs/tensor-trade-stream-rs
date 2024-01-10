use graphql_client::GraphQLQuery;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Decimal(pub String);

#[derive(Debug, Clone, Deserialize)]
pub struct Timestamp(pub i64);

#[derive(Debug, Clone, Deserialize)]
pub struct BigInt(pub String);

#[derive(GraphQLQuery, Debug, Clone, Deserialize)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/subscriptions/tswap_order_update_all.graphql",
    response_derives = "Debug, Clone"
)]
pub struct TswapOrderUpdateAll;

#[derive(GraphQLQuery, Debug, Clone, Deserialize)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/subscriptions/tswap_order_update.graphql",
    response_derives = "Debug, Clone"
)]
pub struct TswapOrderUpdate;

#[derive(GraphQLQuery, Debug, Clone, Deserialize)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/subscriptions/new_transaction_t_v2.graphql",
    response_derives = "Debug, Clone"
)]
pub struct NewTransactionTV2;
