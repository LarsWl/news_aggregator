use serde::{Deserialize, Serialize};
use crate::{
  source::parser::*,
  source::parser::parser::*,
  http_client,
  article::article,
  model,
  source::codes::*,
  repository::Repository
};

use std::collections::HashMap;
use mongodb::bson::Bson;

const COLLECTION_NAME: &str = "sources";

#[derive(Debug, Serialize, Deserialize)]
pub struct Source {
  pub _id: u32,
  pub name: String,
  pub code: String,
  pub rss_feed_url: String
}

impl Source {
  pub async fn sync_articles(&self) -> Vec<article::Article> {
    let articles = self.fetch_articles().await;

    Repository::<article::Article>::new()
      .await
      .save_many(&articles)
      .await
      .unwrap();

    
    articles
  }
  pub async fn fetch_articles(&self) -> Vec<article::Article> {
    let feed = self.fetch_feed().await;
    let parser = self.parser();


    parser.parse_rss(&feed)
  }
  
  pub async fn fetch_feed(&self) -> String {
    http_client::HttpClient::fetch_get_html(self.rss_feed_url.as_str())
      .await
      .expect("failed http request")
  }

  pub fn parser(&self) -> impl parser::Parser {
    let parser = match &self.code.as_str() {
      RBC_CODE => rbc::Rbc(),
      _ => panic!()
    };

    parser
  }
}

impl model::Model for Source {
  fn collection_name() -> String {
    String::from(COLLECTION_NAME)
  }
}

