use std::env;
use yup_oauth2::ServiceAccountKey;
use crate::util::postgres_connection::DatabaseConnection;

pub async fn get_gcp_credentials() ->ServiceAccountKey{
    let db_url = env::var("DATABASE_URL").expect("Database not found");
    let db_connection = DatabaseConnection::new(&db_url).await;
    let destinations = db_connection.get_credentials().await.expect("TODO: panic message");
    let mut  properties=std::default::Default::default();
    let mut credential_properties=std::default::Default::default();
    for results in destinations {
        properties = results.properties;
        credential_properties = results.confidential_properties;
    }
    ServiceAccountKey{
        key_type: Some(properties["cloudJson.type"].as_str().unwrap().to_string()),
        project_id: Some(properties[ "cloudJson.project_id"].as_str().unwrap().to_string()),
        private_key_id: Some(properties["cloudJson.private_key_id"].as_str().unwrap().to_string()),
        private_key: credential_properties["cloudJson.private_key"].as_str().unwrap().to_string(),
        client_email: properties["cloudJson.client_email"].as_str().unwrap().to_string(),
        client_id: Some(properties["cloudJson.client_id"].as_str().unwrap().to_string()),
        auth_uri: Some(properties["cloudJson.auth_uri"].as_str().unwrap().to_string()),
        token_uri: properties["cloudJson.token_uri"].as_str().unwrap().to_string(),
        auth_provider_x509_cert_url: Some(properties["cloudJson.auth_provider_x509_cert_url"].as_str().unwrap().to_string()),
        client_x509_cert_url: Some(properties["cloudJson.client_x509_cert_url"].as_str().unwrap().to_string()),
    }
}
