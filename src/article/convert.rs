use crate::{
  article::article::Article,
  source::parser::rbc::RbcItem,
  source::parser::rss_structure::RssItem
};
use md5;

impl From<RbcItem> for Article {
  fn from(item: RbcItem) -> Article {
    let document_id = md5::compute(RbcItem::source_code() + &item.title + &item.publication_date);
    
    Article::new(
      format!("{:X}", document_id),
      item.title,
      item.full_text,
      item.description,
      item.publication_date,
      RbcItem::source_code()
    )
  }
}