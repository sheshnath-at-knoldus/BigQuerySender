use std::borrow::Cow;
use crate::model;
use crate::events::{
    event_value, BrokerType, EventData, EventHeader, EventValue, ValueMap, ValueVector,
};
use protobuf::descriptor::field_options::JSType;
use protobuf::rt::CachedSize;
use protobuf::well_known_types::struct_::Value;
use protobuf::{CodedInputStream, Message, MessageField, SpecialFields, UnknownFields};
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use serde_json::{from_slice, json};
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::str::from_utf8;
use log::log;
use serde::__private::from_utf8_lossy;
use serde::de::Unexpected::Str;
use crate::ingestion::{BrokerMessage, IngestionHeader};
fn process_data(data :Cow<str>) -> serde_json::Value {
    let data =data.as_ref();
    let json_start= data.find('{').unwrap();
    let json_end=data.rfind('}').unwrap();
    let json= &data[json_start..=json_end];
    let data : serde_json::Value = serde_json::from_str(json).expect("");
    data
}
pub(crate) fn deserializer(payload: &[u8]) -> serde_json::Value {
    //
    // let mut input_stream = CodedInputStream::from_bytes(payload);
    // let mut event_data = EventData::new();
    //     event_data
    //     .merge_from(&mut input_stream)
    //     .expect("error while parsing bytes from Kafka");
    // let json_data = parse_event_data(event_data.clone());
    // //println!("json_data{}",json_data.clone());
    // json_data

    let cow_Str=from_utf8_lossy(payload);
    process_data(cow_Str)
}

fn parse_event_data(event_data: EventData) -> serde_json::Value {
    // Create a JSON object to hold the parsed EventData fields
    let mut event_data_json = json!({});
    let event_id = event_data.header.event_id.clone();
    let device_id = event_data.header.device_id.clone();
    let tenant_id = event_data.header.tenant_id.clone();
    let message_id = event_data.header.message_id.clone();
    let tags = event_data.header.tags.clone();
    let ingestion_timestamp = event_data.header.ingestion_timestamp.clone();
    let broker_type = event_data.header.broker_type.unwrap();
    let body = parse_value_map(event_data.body.unwrap());
    let header_json = json!( {
        "event_id": event_id,
        "tenant_id":tenant_id,
        "device_id":device_id,
        "message_id":message_id,
        "tags":tags,
        "ingestion_timestamp":ingestion_timestamp,
    });
    event_data_json["header"] = header_json;
    event_data_json["body"] = body;
    // Return the parsed EventData as JSON value
    event_data_json
}

fn parse_event_value(event_value: EventValue) -> serde_json::Value {
    match event_value.value {
        Some(value) => match value {
            event_value::Value::StringV(string_data) => json!(string_data),
            event_value::Value::IntV(int_data) => json!(int_data),
            event_value::Value::DoubleV(double_data) => json!(double_data),
            event_value::Value::BooleanV(boolean_data) => json!(boolean_data),
            event_value::Value::DateTimeV(date_time) => json!(date_time),
            event_value::Value::DurationV(duration) => json!(duration),
            event_value::Value::VectorV(vector_value) => {
                let mut vec_values = Vec::new();
                for value in vector_value.values {
                    vec_values.push(parse_event_value(value));
                }
                json!(vec_values)
            }
            event_value::Value::MapV(map_value) => parse_value_map(map_value),
        },
        None => json!(null),
    }
}

fn parse_value_map(value_map: ValueMap) -> serde_json::Value {
    let mut map_values = HashMap::new();
    for (key, event_value) in value_map.values {
        map_values.insert(key, parse_event_value(event_value));
    }
    json!(map_values)
}

