use hocon::HoconLoader;
use lazy_static::lazy_static;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub project_id: String,
    pub dataset_id: String,
    pub table_id: String,
    pub kafka_broker:String,
    pub topic:String,
    pub group_id:String,
}

lazy_static! {
    pub static ref CONFIG: Config = get_config();
}

fn get_config() -> Config {
    let config: Config = HoconLoader::new()
        .load_file("src/resources/application.config")
        .expect("unable to load config file")
        .resolve()
        .expect("config deserialize error");
    config
}
