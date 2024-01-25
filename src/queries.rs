use graphql_client::GraphQLQuery;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Decimal(pub String);

impl Decimal {
    pub fn new(decimal: String) -> Self {
        Self(decimal)
    }
}

impl From<String> for Decimal {
    fn from(item: String) -> Self {
        Self::new(item)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Timestamp(pub i64);

impl Timestamp {
    pub fn new(timestamp: i64) -> Self {
        Self(timestamp)
    }
}

impl From<i64> for Timestamp {
    fn from(item: i64) -> Self {
        Self::new(item)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct BigInt(pub String);

impl BigInt {
    pub fn new(big_int: String) -> Self {
        Self(big_int)
    }
}

impl From<String> for BigInt {
    fn from(item: String) -> Self {
        Self::new(item)
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
