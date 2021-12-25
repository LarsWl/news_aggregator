use crate::{
  config
};

use mongodb::{ 
  Client,
  Database,
  options::ClientOptions,
};

pub struct Connection {
  pub client: Client,
  pub database: Database
}

pub async fn create_connection() -> Result<Connection,  Box<dyn std::error::Error>> {
  let client = Client::with_options(mongo_client_options().await?)?;
  let config = config::Config::new();

  Ok(Connection {
    database: client.database(config.database_name()),
    client
  })
}

async fn mongo_client_options() -> Result<ClientOptions, Box<dyn std::error::Error>> {
  let config = config::Config::new();

  Ok(ClientOptions::parse(config.mongodb_uri()).await?)
}