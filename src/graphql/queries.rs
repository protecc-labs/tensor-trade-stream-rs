use graphql_client::GraphQLQuery;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Decimal(f64);

#[derive(Debug, Deserialize)]
struct Timestamp(i64);

#[derive(Debug, Deserialize)]
struct BigInt(String);

#[derive(GraphQLQuery, Debug, Deserialize)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/subscriptions/tswap_order_update_all.graphql",
    response_derives = "Debug"
)]
pub struct TswapOrderUpdateAll;

#[derive(GraphQLQuery, Debug, Deserialize)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/subscriptions/tswap_order_update.graphql",
    response_derives = "Debug"
)]
pub struct TswapOrderUpdate;

#[derive(GraphQLQuery, Debug, Deserialize)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/subscriptions/new_transaction_t_v2.graphql",
    response_derives = "Debug"
)]
pub struct NewTransactionTV2;
