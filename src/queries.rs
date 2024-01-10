use graphql_client::GraphQLQuery;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Decimal(String);

impl Into<String> for Decimal {
    fn into(self) -> String {
        self.0
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Timestamp(i64);

impl Into<i64> for Timestamp {
    fn into(self) -> i64 {
        self.0
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct BigInt(String);

impl Into<String> for BigInt {
    fn into(self) -> String {
        self.0
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
