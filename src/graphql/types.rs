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
    query_path = "graphql/subscriptions/tensorswap_pool_updates.graphql",
    response_derives = "Debug"
)]
pub struct TswapOrderUpdateAll;
