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
      Err(_) => {
        return vec![];
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

#[cfg(test)]
mod tests {
  use crate::rss::parser::Parser;
  use crate::rss::rss_item::RssItem;
  use crate::article::article::Article;
  use std::fs;

  const SOURCE_CODE: &str = "test";
  const INCORRECT_XML_PATH: &str = "test/test_data/incorrect_rss_xml.xml";
  const CORRECT_XML_PATH: &str = "test/test_data/correct_rss_xml.xml";

  #[test]
  fn return_empty_vec_if_error_in_xml() {
    let parser = Parser::new(SOURCE_CODE);
    let xml = fs::read_to_string(INCORRECT_XML_PATH).unwrap();

    let articles = parser.parse_rss(&xml);

    assert_eq!(articles.len(), 0);
  }

  #[test]
  fn return_vector_with_data_from_xml() {
    let parser = Parser::new(SOURCE_CODE);
    let xml = fs::read_to_string(CORRECT_XML_PATH).unwrap();

    let articles = parser.parse_rss(&xml);


    assert_eq!(articles.len(), 1);
    assert_eq!(articles[0], correct_article());
  }

  fn correct_article() -> Article {
    let rss_item = RssItem::new(
        "https://www.finanz.ru/novosti/obligatsii/banki-besplatno-razmeshchayut-evrobondy-em-v-pogone-za-doley-rynka-1001127032".to_string(),
        "Банки бесплатно размещают евробонды ЕМ в погоне за долей рынка".to_string(),
        "Thu, 31 Mar 2016 15:57:25 GMT".to_string(),
        "Государственная энергокомпания Индии NTPC Ltd. в феврале разместила еврооблигации на $500 миллионов, заплатив в качестве комиссии всего пару центов вместо более чем $1 миллиона".to_string(),
        "None".to_string(),
        "https://www.finanz.ru/novosti/obligatsii/banki-besplatno-razmeshchayut-evrobondy-em-v-pogone-za-doley-rynka-1001127032".to_string()
    );

    Article::from_rss_item(rss_item, SOURCE_CODE)
  }
}