use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Rss<T: Default + RssItem> {
  pub channel: Channel<T>
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Channel<T: Default + RssItem> {
  #[serde(rename = "item", default)]
  pub items: Vec<T>
}

pub trait RssItem {
  fn source_code() -> String;
}