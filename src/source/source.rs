use crate::{
  http_client,
  article::article,
  model,
  repository::Repository,
  rss::parser::Parser
};
use serde::{Deserialize, Serialize};

const COLLECTION_NAME: &str = "sources";

#[derive(Debug, Serialize, Deserialize)]
pub struct Source {
  pub id: u32,
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
      .unwrap_or_else( |err| {
        match *err.kind {
          mongodb::error::ErrorKind::BulkWrite(e) => println!("Skip some duplicates"),
          mongodb::error::ErrorKind::Authentication { message: e, .. } => panic!("{}", e),
          mongodb::error::ErrorKind::InvalidArgument { message: e, .. } => println!("There is not articles fetched"),
          e => panic!("There something wrong with save articles, {:#?}", e)
        }
      });

    
    articles
  }
  async fn fetch_articles(&self) -> Vec<article::Article> {
    let feed = self.fetch_feed().await;
    let parser = self.parser();


    parser.parse_rss(&feed)
  }
  
  async fn fetch_feed(&self) -> String {
    http_client::HttpClient::fetch_get_html(self.rss_feed_url.as_str())
      .await
      .unwrap_or_else(|_| { "".to_string() })
  }

  fn parser(&self) -> Parser {
    Parser::new(self.code.as_str())
  }
}

impl model::Model for Source {
  fn collection_name() -> String {
    String::from(COLLECTION_NAME)
  }
}

#[cfg(test)]
mod tests {
  use crate::source::source::Source;
  use crate::repository::Repository;
  use crate::article::article::Article;
  use std::collections::HashMap;
  use httpmock::prelude::*;
  use std::fs;
  use mongodb::bson::Bson;

  const TEST_CODE: &str = "test";
  const CORRECT_TEST_RSS_FEED_PATH: &str = "/rss.xml";
  const INCORRECT_TEST_RSS_FEED_PATH: &str = "/bad_rss.xml";
  const CORRECT_XML_PATH: &str = "test/test_data/correct_rss_xml.xml";

  #[tokio::test]
  async fn sync_articles_return_empty_vec_if_there_error_in_rss_request() {
    dotenv::from_filename(".env.test").ok();

    let server = mock_server();
    let source = test_source(&server, INCORRECT_TEST_RSS_FEED_PATH);

    let articles = source.sync_articles().await;

    assert_eq!(articles.len(), 0)
  }

  #[tokio::test]
  async fn sync_articles_return_parsed_articles() {
    dotenv::from_filename(".env.test").ok();

    let server = mock_server();
    let source = test_source(&server, CORRECT_TEST_RSS_FEED_PATH);

    let articles = source.sync_articles().await;

    assert_eq!(articles.len(), 1)
  }

  #[tokio::test]
  async fn sync_articles_save_articles_to_mongodb() {
    dotenv::from_filename(".env.test").ok();

    let server = mock_server();
    let source = test_source(&server, CORRECT_TEST_RSS_FEED_PATH);

    let articles = source.sync_articles().await;
    let mut options = HashMap::<String, Bson>::new();
    options.insert("document_id".to_string(), Bson::String(articles[0].document_id.clone()));

    let articles_from_mongo = Repository::<Article>::new().await.find(options).await;

    assert_eq!(articles[0], articles_from_mongo[0])
  }


  fn test_source(mock_server: &MockServer, rss_feed_url: &str) -> Source {
    Source {
      id: 1,
      name: "test".to_string(),
      code: TEST_CODE.to_string(),
      rss_feed_url: mock_server.url(rss_feed_url)
    }
  }

  fn mock_server() -> MockServer {
    let server = MockServer::start();

    server.mock(|when, then| {
      when.method(GET)
          .path(CORRECT_TEST_RSS_FEED_PATH);
      then.status(200)
          .header("content-type", "text/html")
          .body(fs::read_to_string(CORRECT_XML_PATH).unwrap());
    });

    server.mock(|when, then| {
      when.method(GET)
          .path(INCORRECT_TEST_RSS_FEED_PATH);
      then.status(404);
    });

    server
  }
}

