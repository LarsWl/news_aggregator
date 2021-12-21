use crate::source::parser::parser;
use crate::article::article;
use crate::source::parser::rss_structure::*;

extern crate serde;
extern crate quick_xml;

use serde::Deserialize;
use quick_xml::de::{ from_str };

pub struct Rbc();

const SOURCE_CODE: &str = "rbc";

impl parser::Parser for Rbc {
  fn parse_rss(&self, xml: &str) -> Vec<article::Article> {
    let rss: Rss<RbcItem> = from_str(xml).expect("Error happened when parsing RBC rss");

    rss.channel
      .items
      .into_iter()
      .map( |item| article::Article::from(item) )
      .collect()
  }
}

// Описание RssItem

#[derive(Debug, Deserialize, PartialEq)]
pub struct RbcItem {
  #[serde(rename = "rbc_news:news_id", default)]
  pub document_id: String,
  pub title: String,
  #[serde(rename = "pubDate", default)]
  pub publication_date: String,
  pub description: String,
  #[serde(rename = "rbc_news:full-text", default)]
  pub full_text: String
}

impl Default for RbcItem {
  fn default() -> Self {
    Self {
      document_id: String::from("default"),
      title: String::from("default"),
      publication_date: String::from("default"),
      description: String::from("default"),
      full_text: String::from("default")
    }
  }
}

impl RssItem for RbcItem {
  fn source_code() -> String {
    String::from(SOURCE_CODE)
  }
}
