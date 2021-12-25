use crate::{
  model,
  rss::rss_item::RssItem
};
use serde::{Serialize, Deserialize};

pub const COLLECTION_NAME: &str = "articles";

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Article {
  pub document_id: String,
  pub title: String,
  pub content: String,
  pub description: String,
  pub source_code: String,
  pub publication_date: String,
  pub link: String
}

impl Article {
  pub fn new(
    document_id: String, 
    title: String, 
    content: String, 
    description: String,
    publication_date: String,
    link: String,
    source_code: String
  ) -> Article 
  {
    Article {
      document_id,
      title,
      content,
      description,
      source_code,
      publication_date,
      link
    }
  }

  pub fn from_rss_item(rss_item: RssItem, source_code: &str) -> Article {
    Article::new(
      rss_item.document_id, 
      rss_item.title,
      rss_item.full_text,
      rss_item.description,
      rss_item.publication_date,
      rss_item.link,
      source_code.to_string()
    )
  }
}

impl model::Model for Article {
  fn collection_name() -> String {
    String::from(COLLECTION_NAME)
  }
}