use std::env;
use mongodb::{ 
  Client,
  Database,
  options::ClientOptions,
};

const DATABASE_NAME: &str = "news_aggregator";

pub struct Connection {
  pub client: Client,
  pub database: Database
}

pub async fn create_connection() -> Result<Connection,  Box<dyn std::error::Error>> {
  let client = Client::with_options(mongo_client_options().await?)?;

  Ok(Connection {
    database: client.database(DATABASE_NAME),
    client
  })
}

async fn mongo_client_options() -> Result<ClientOptions, Box<dyn std::error::Error>> {
  Ok(ClientOptions::parse(mongodb_uri()).await?)
}

fn mongodb_uri() -> String {
  env::var("MONGODB_URI").expect("MONGODB_URI env var should be specified")
}