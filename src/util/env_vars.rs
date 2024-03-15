use dotenv::dotenv;
use std::env;

pub(crate) fn env_vars() -> (String,String) {
    dotenv().ok();
    // let project_id = env::var("PROJECT_ID").expect("Environment variable PROJECT_ID");
    // let dataset_id = env::var("DATASET_ID").expect("Environment variable DATASET_ID");
    // let table_id = env::var("TABLE_ID").expect("Environment variable TABLE_ID");
    let gcp_sa_key = env::var("GOOGLE_APPLICATION_CREDENTIALS")
        .expect("Environment variable GOOGLE_APPLICATION_CREDENTIALS");
    // let group_id = env::var("GROUP_ID").expect("UNABLE TO GET GROUP ID");
    // let broker = env::var("KAFKA_BROKER").expect("UNABLE TO GET KAFKA BROKER");
     let topic = env::var("TOPIC").expect("UNABLE TO GET TOPIC");
    (gcp_sa_key,topic)
}
