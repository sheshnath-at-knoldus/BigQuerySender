mod consumer;
mod model;
mod util;
mod big_query;

use crate::consumer::kafka_consumer::kafka_consumer;
use crate::model::model::BQTable;
use dotenv::dotenv;
use gcp_bigquery_client::model::dataset::Dataset;
use gcp_bigquery_client::model::query_request::QueryRequest;
use gcp_bigquery_client::model::table::Table;
use gcp_bigquery_client::model::table_data_insert_all_request::TableDataInsertAllRequest;
use gcp_bigquery_client::model::table_field_schema::TableFieldSchema;
use gcp_bigquery_client::model::table_schema::TableSchema;
use gcp_bigquery_client::Client;
use log::info;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{BaseConsumer, Consumer};
use rdkafka::message::Message;
use serde::Deserialize;
use serde::Serialize;
use std::env;
use std::fmt::Debug;
use std::sync::mpsc;

mod events {
    include!(concat!(env!("OUT_DIR"), "/events.rs"));
}
mod ingestion {
    include!(concat!(env!("OUT_DIR"), "/ingestion.rs"));
}



use crate::big_query::big_query:: {create_data_set, insert_into_big_query};
use events::EventData;
use util::{
    env_vars::env_vars, kafka_deserializer::deserializer,
};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let (
        ref project_id,
        ref dataset_id,
        ref table_id,
        ref gcp_sa_key,
        ref group_id,
        ref broker,
        ref topic,
    ) = env_vars();

    // Kafka broker address

    let table_data_insert_request = TableDataInsertAllRequest::new();
    let big_query_client = Client::from_service_account_key_file(gcp_sa_key)
        .await
        .expect("unable to fetch data");

    let mut consumer_config = ClientConfig::new();
    consumer_config
        .set("group.id", group_id)
        .set("bootstrap.servers", broker);

    info!("before kafka_consumer");

    kafka_consumer(
        consumer_config,
        &topic,
        big_query_client,
        table_data_insert_request,
    )
    .await;

}
