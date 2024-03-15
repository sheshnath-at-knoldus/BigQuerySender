mod consumer;
mod model;
mod util;
mod big_query;
use crate::consumer::kafka_consumer::kafka_consumer;
use crate::model::model::{BQTable, Object};
use crate::util::{config::CONFIG, postgres_connection::DatabaseConnection};
use dotenv::dotenv;
use gcp_bigquery_client::model::dataset::Dataset;
use gcp_bigquery_client::model::query_request::QueryRequest;
use gcp_bigquery_client::model::table::Table;
use gcp_bigquery_client::model::table_data_insert_all_request::TableDataInsertAllRequest;
use gcp_bigquery_client::model::table_field_schema::TableFieldSchema;
use gcp_bigquery_client::model::table_schema::TableSchema;
use gcp_bigquery_client::Client;
use log::{debug, info};
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{BaseConsumer, Consumer};
use rdkafka::message::Message;
use serde::Deserialize;
use serde::Serialize;
use std::env;
use std::fmt::Debug;
use std::sync::mpsc;
use crate::big_query::gcp_credentials::get_gcp_credentials;
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

use env_logger;
use rdkafka::admin::ConfigSource::Default;
use serde_json::Value;
use yup_oauth2::ServiceAccountKey;

#[tokio::main]
async fn main() {
    env_logger::init();
    let (ref gcp_sa_key,ref topic)= env_vars();
    let gcs:ServiceAccountKey=get_gcp_credentials().await;

    //Kafka broker address
    let table_data_insert_request = TableDataInsertAllRequest::new();
    let big_query_client = Client::from_service_account_key(gcs,false)
        .await
        .expect("unable to fetch data");
    let mut consumer_config = ClientConfig::new();
    consumer_config
        .set("group.id", &*CONFIG.group_id)
        .set("bootstrap.servers", &*CONFIG.kafka_broker);
    kafka_consumer(
        consumer_config,
        &*CONFIG.topic,
        big_query_client,
        table_data_insert_request,
    )
    .await;

}
