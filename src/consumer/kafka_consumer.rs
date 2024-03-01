use crate::util::env_vars::env_vars;
use crate::util::kafka_deserializer::deserializer;
use crate::big_query::big_query::insert_into_big_query;
use gcp_bigquery_client::model::table_data_insert_all_request::TableDataInsertAllRequest;

use gcp_bigquery_client::model::table_data_insert_all_response::TableDataInsertAllResponse;
use rdkafka::consumer::{BaseConsumer, Consumer};
use rdkafka::{ClientConfig, Message};
use std::time::Duration;
use serde::__private::from_utf8_lossy;

pub(crate) async fn kafka_consumer(
    client_config: ClientConfig,
    topic: &str,
    big_query_client: gcp_bigquery_client::Client,
    table_data_insert_request: TableDataInsertAllRequest,
) {
    let consumer: BaseConsumer = client_config.create().expect("Consumer creation failed");
    // Subscribe to the topic
    consumer
        .subscribe(&[topic])
        .expect("Subscription to topic failed");

    // Start consuming messages
    loop {
        match consumer.poll(Duration::from_millis(2000)) {
            Some(Ok(message)) => {
                if let Some(payload) = message.detach().payload() {
                   // println!("payload{:#?}",from_utf8_lossy(payload));
                   println!("Running");
                   insert_deserialized_data_into_big_query(
                        big_query_client.clone(),
                        table_data_insert_request.clone(),
                        deserializer(payload),
                    )
                    .await;
                } else {
                    eprintln!("Unable to get data from message");
                }
            }
            Some(Err(err)) => eprintln!("Error: {:?}", err),
            _ => {}
        }
    }
}

pub(crate) async fn insert_deserialized_data_into_big_query(
    big_query_client: gcp_bigquery_client::Client,
    table_data_insert_request: TableDataInsertAllRequest,
    value: serde_json::Value,
) -> TableDataInsertAllResponse {
    let (
        ref project_id,
        ref dataset_id,
        ref table_id,
        ref gcp_sa_key,
        ref group_id,
        ref broker,
        ref topic,
    ) = env_vars();
     insert_into_big_query(
        table_data_insert_request,
        &big_query_client,
        project_id,
        dataset_id,
        table_id,
        value,
    )
    .await

}
