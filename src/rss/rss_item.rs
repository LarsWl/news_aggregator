use md5;

pub struct RssItem {
  pub document_id: String,
  pub title: String,
  pub description: String,
  pub full_text: String,
  pub publication_date: String,
  pub link: String
}

impl RssItem {
  pub fn new(guid: String, title: String, description: String, full_text: String, publication_date: String, link: String) -> RssItem {
    let document_id = md5::compute(guid + &title + &description + &publication_date);

    RssItem {
      document_id: format!("{:?}", document_id),
      title,
      description,
      publication_date,
      full_text,
      link
    }
  }
}