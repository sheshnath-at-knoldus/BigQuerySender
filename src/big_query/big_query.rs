use gcp_bigquery_client::model::dataset::Dataset;
use gcp_bigquery_client::model::field_type::FieldType;
use gcp_bigquery_client::model::table::Table;
use gcp_bigquery_client::model::table_data_insert_all_request::TableDataInsertAllRequest;
use gcp_bigquery_client::model::table_data_insert_all_response::TableDataInsertAllResponse;
use gcp_bigquery_client::model::table_field_schema::TableFieldSchema;
use gcp_bigquery_client::model::table_schema::TableSchema;
use gcp_bigquery_client::Client;
use serde_json::Map;
use std::collections::HashMap;

pub(crate) async fn insert_into_big_query(
    mut insert_request: TableDataInsertAllRequest,
    client_config: &Client,
    project_id: &str,
    data_set: &str,
    table_id: &str,
    value: serde_json::Value,
) -> TableDataInsertAllResponse {
    create_data_set(
        &client_config,
        &project_id.to_string(),
        &data_set.to_string(),
        &table_id.to_string(),
        value.clone(),
    )
    .await;

    let mut flattened = Map::new();

    flatten_json(value.clone(), "", &mut flattened);

    insert_request
        .add_row(None, flattened)
        .expect("unable to insert data ");

    let response = client_config
        .tabledata()
        .insert_all(project_id, data_set, table_id, insert_request)
        .await
        .expect("unable to insert data ");
    println!("Insert error ->  {:?}", response.insert_errors);
    response
}

fn create_schema(json_data: serde_json::Value) -> Vec<TableFieldSchema> {
    let json_value = json_data;
    let mut flat_schema = HashMap::new();
    flatten_json_for_schema("", json_value, &mut flat_schema);
    flat_schema
        .into_iter()
        .map(|(key, value)| TableFieldSchema {
            name: key.clone(),
            r#type: get_field_type(value.clone()),
            mode:     if value.is_array() {
                Some("REPEATED".to_string())
            } else {
                None
            },
            fields: None,
            description: None,
            categories: None,
            policy_tags: None,
        })
        .collect()
}

fn flatten_json(
    json: serde_json::Value,
    prefix: &str,
    flattened: &mut Map<String, serde_json::Value>,
) {
    match json {
        serde_json::Value::Object(obj) => {
            for (key, value) in obj {
                let new_prefix = if prefix.is_empty() {
                    key.clone()
                } else {
                    format!("{}_{}", prefix, key)
                };
                flatten_json(value, &new_prefix, flattened);
            }
        }
        _ => {
            flattened.insert(prefix.to_string(), json.clone());
        }
    }
}
fn flatten_json_for_schema(
    prefix: &str,
    value: serde_json::Value,
    schema: &mut HashMap<String, serde_json::Value>,
) {
    match value {
        serde_json::Value::Object(obj) => {
            for (key, val) in obj {
                let key_with_prefix = if prefix.is_empty() {
                    key.clone()
                } else {
                    format!("{}_{}", prefix, key)
                };
                flatten_json_for_schema(&key_with_prefix, val, schema);
            }
        }
        _ => {
            schema.insert(prefix.to_string(), value.clone());
        }
    }
}

fn get_field_type(value: serde_json::Value) -> FieldType {
    match value {
        serde_json::Value::Bool(_) => FieldType::Boolean,
        serde_json::Value::Number(_) => FieldType::Float64,
        serde_json::Value::String(_) => FieldType::String,
        serde_json::Value::Array(data) => {
            let mut vec = Vec::new();
            for value in data {
                vec.push(get_field_type(value));
            }
            let mut field = FieldType::String;
            for data in vec {
                field = data
            }
            return field;
        }
        serde_json::Value::Object(_) => FieldType::Record,
        _ => FieldType::String,
    }
}

pub(crate) async fn create_data_set(
    big_query_client: &Client,
    project_id: &String,
    dataset_id: &String,
    table_id: &String,
    value: serde_json::Value,
) {
    let table_exists = big_query_client
        .table()
        .get(project_id, dataset_id, table_id, None)
        .await
        .is_ok();

    if !table_exists {
        // Create a new dataset
        let dataset = big_query_client
            .dataset()
            .create(
                Dataset::new(project_id, dataset_id)
                    .location("US")
                    .friendly_name("Just a demo dataset")
                    .label("owner", "me")
                    .label("env", "prod"),
            )
            .await
            .expect("unable to create a dataset");
        create_table(dataset,big_query_client.clone(),table_id,value.clone()).await;
    } else {
        update_table_schema(big_query_client.clone(), project_id, dataset_id, table_id, value.clone()).await;
    }
}


async fn create_table(dataset: Dataset, big_query_client :Client, table_id :&String,value :serde_json::Value) {
    let table_schema= create_schema(value);
    dataset
        .create_table(
            &big_query_client,
            Table::from_dataset(&dataset, table_id, TableSchema::new(table_schema))
                .friendly_name("Demo table")
                .description("a nice description")
                .label("owner", "me"),
        )
        .await
        .expect("unable to create a Table");
}

async fn update_table_schema(big_query_client :Client, project_id:&String, dataset_id:&String, table_id:&String,value :serde_json::Value) {
    let mut table = big_query_client
        .table()
        .get(project_id, dataset_id, table_id, None)
        .await
        .expect("unable to get table");

    let existing_table_schema = table.schema.fields.clone().unwrap_or_else(|| vec![]);
    let mut updated_table_schema = existing_table_schema.clone();

    let new_fields = create_schema(value.clone());

    // Check if any new fields are found
    let mut new_fields_found = false;

    for new_field in new_fields {
        if !existing_table_schema.iter().any(|field| field.name == new_field.name) {
            println!("New field found: {:?}", new_field.clone());
            updated_table_schema.push(new_field);
            new_fields_found = true;
        }
    }
    if new_fields_found {
        println!("New column(s) added");

        // Update the table schema only if new fields are found
        table.schema.fields = Some(updated_table_schema);
        let _ = big_query_client
            .table()
            .update(project_id, dataset_id, table_id, table)
            .await
            .expect("unable to update table schema");
    } else {}
}
