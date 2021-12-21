use serde::{Serialize, Deserialize};
use crate::model;

pub const COLLECTION_NAME: &str = "articles";

#[derive(Debug, Serialize, Deserialize)]
pub struct Article {
  pub _id: String,
  pub title: String,
  pub content: String,
  pub description: String,
  pub source_code: String,
  pub publication_date: String
}

impl Article {
  pub fn new(
    _id: String, 
    title: String, 
    content: String, 
    description: String,
    source_code: String,
    publication_date: String
  ) -> Article 
  {
    Article {
      _id,
      title,
      content,
      description,
      source_code,
      publication_date
    }
  }
}

impl model::Model for Article {
  fn collection_name() -> String {
    String::from(COLLECTION_NAME)
  }
}