use crate::{
  article::article::Article,
  rss::rss_item::RssItem
};
use rss::Channel;

pub struct Parser {
    source_code: String
}

impl Parser {
  pub fn new(code: &str) -> Parser {
    Parser { source_code: code.to_string() }
  }

  pub fn parse_rss(&self, xml: &str) -> Vec<Article> {
    match Channel::read_from(xml.as_bytes()) {
      Ok(channel) => self.fetch_from_channel(channel),
      Err(err) => {
        panic!("{}", err)
      }
    }
  }

  fn fetch_from_channel(&self, channel: Channel) -> Vec<Article> {
    let items: Vec<RssItem> = channel
      .items()
      .iter()
      .map(|item| {
        let guid = match item.guid() {
          Some(guid) => guid.value().to_string(),
          None => "None".to_string()
        };

        RssItem::new(
          guid,
          item.title().unwrap_or_else(|| "None" ).to_string(),
          item.pub_date().unwrap_or_else(|| "None" ).to_string(),
          item.description().unwrap_or_else(|| "None" ).to_string(),
          item.content().unwrap_or_else(|| "None" ).to_string(),
          item.link().unwrap_or_else(|| "None" ).to_string()
        )
      }).collect();

    let articles: Vec<Article> = items.into_iter()
      .map(|item| {
        Article::from_rss_item(item, self.source_code.as_str())
      })
      .collect();

    articles
  }
}