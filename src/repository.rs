use crate::{
  db::connection,
  model
};

use serde::{
  Serialize, de::DeserializeOwned,};
use std::collections::HashMap;
use mongodb::{bson::Bson, options::FindOptions};
use std::marker::Unpin;
use futures::TryStreamExt;


pub struct Repository<T> 
where 
  T: model::Model + Send + Sync + Serialize + DeserializeOwned + Unpin
{
  collection: mongodb::Collection<T>
}

impl<T> Repository<T>
where
  T: model::Model + Send + Sync + Serialize + DeserializeOwned + Unpin
{
  pub async fn new() -> Repository<T> {
    let connection = connection::create_connection()
        .await
        .expect("Couldnt create connection to MongoDb");
    let collection = connection.database.collection::<T>(T::collection_name().as_str());

    Repository {
      collection
    }
  }

  pub async fn save(&self, model: &T) -> Result<(), Box<dyn std::error::Error>> {
    self.collection.insert_one(model, None).await?;

    Ok(())
  }

  pub async fn save_many(&self, models: &Vec<T>) -> Result<(), Box<dyn std::error::Error>> {
    self.collection.insert_many(models, None).await?;

    Ok(())
  }

  pub async fn find(&self, filter_args: HashMap<String, Bson>) -> Vec<T> {
    let filter: mongodb::bson::Document = filter_args.into_iter().collect();
    let find_options = FindOptions::builder().build();
    let cursor = self.collection.find(filter, find_options).await.unwrap();

    cursor.try_collect().await.unwrap()
  }
}