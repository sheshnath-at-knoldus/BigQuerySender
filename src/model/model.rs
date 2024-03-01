use chrono::{DateTime, Utc};
use protobuf::well_known_types::duration::Duration;
use serde::Serialize;
use serde_json::Map;
use std::collections::HashMap;
use std::string::String;

#[derive(Serialize)]
pub(crate) struct BQTable {
    pub message_id: String,
    pub event_id: String,
    pub device_id: String,
    pub received_at: String,
    pub triggered_rule: String,
    pub delivered_at: String,
}

pub(crate) struct EventData {
    pub header: EventHeader,
    pub body: ValueMap,
}

pub(crate) struct EventHeader {
    pub tenant_id: String,
    pub device_id: String,
    pub message_id: String,
    pub tags: String,
    pub ingestion_timstamp: DateTime<Utc>,
    pub broker_type: BrokerType,
    pub event_id: String,
}

pub(crate) enum BrokerType {
    DANLAW_TCP,
    DANLAW_UDP,
    HTTP,
    BOSCH,
}
#[derive(Serialize)]
pub(crate) struct Data {
    pub(crate) data: serde_json::Value,
}

#[derive(Debug)]
enum Value {
    IntV(i64),
    DoubleV(f64),
    StringV(String),
    BooleanV(bool),
    DurationV(Duration),
    DateTimeV(String),
    VectorV(Vec<Value>),
    MapV(HashMap<String, Value>),
    Empty,
}

pub(crate) struct EventValue {
    value: Value,
}

pub(crate) struct ValueVector {
    repeated: EventValue,
}

pub(crate) struct ValueMap {
    pub values: Map<String, EventValue>,
}
