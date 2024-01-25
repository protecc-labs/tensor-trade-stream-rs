use graphql_client::GraphQLQuery;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Decimal(String);

impl From<Decimal> for String {
    fn from(decimal: Decimal) -> Self {
        decimal.0
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Timestamp(i64);

impl From<Timestamp> for i64 {
    fn from(timestamp: Timestamp) -> Self {
        timestamp.0
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct BigInt(String);

impl From<BigInt> for String {
    fn from(big_int: BigInt) -> Self {
        big_int.0
    }
}

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
