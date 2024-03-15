use chrono::{DateTime, Utc};
use protobuf::well_known_types::duration::Duration;
use serde::{Deserialize, Serialize};
use serde_json::Map;
use std::collections::HashMap;
use std::string::String;
use sqlx::types::Text;
#[derive(Debug)]
pub struct Object {
   pub cloudJson_auth_provider_x509_cert_url: String,
   pub cloudJson_auth_uri: String,
   pub cloudJson_client_email: String,
   pub cloudJson_client_id: String,
   pub cloudJson_client_x509_cert_url: String,
   pub cloudJson_private_key_id: String,
   pub cloudJson_project_id: String,
   pub cloudJson_token_uri: String,
   pub cloudJson_type: String,
   pub instanceId: String,
   pub tableName: String
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Destinations{
    pub  tenant_id:String,
    pub name:String ,
    pub type_ :String,
    pub properties:serde_json::Value,
    pub confidential_properties :serde_json::Value,

}
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
