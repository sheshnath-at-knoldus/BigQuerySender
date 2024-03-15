use std::error::Error;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres,Row};
use crate::model::model::Destinations;


#[derive(Clone, Debug)]
pub struct DatabaseConnection {
    pool: Pool<Postgres>,
}
impl DatabaseConnection {
    pub async fn new(url: &str) -> DatabaseConnection {
        let pool = sqlx::postgres::PgPool::connect(url).await.expect("error in connections");
        DatabaseConnection { pool }
    }
   pub async fn get_credentials(&self) ->Result<Vec<Destinations>, Box<dyn Error>> {
        let select_query=sqlx::query("Select*from destinations where name ='Bigquery'");
        let rows = select_query.fetch_all(&self.pool).await.expect("");
       let destination = rows.into_iter().map(
           |row| Destinations{

               tenant_id: row.get("tenant_id"),
               name: row.get("name"),
               type_: row.get("type"),
               properties: row.get("properties"),
               confidential_properties: row.get("confidential_properties"),

           }
       ).collect();
    Ok(destination)
    }

}