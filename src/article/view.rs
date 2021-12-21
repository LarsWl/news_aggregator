use serde::Serialize;
use actix_web::{HttpRequest, HttpResponse, Responder};
use serde_json;

#[derive(Serialize)]
pub struct ArticleView {
  title: String,
  description: String,
  content: String
}

impl Responder for ArticleView {
  fn respond_to(self, _req: &HttpRequest) -> HttpResponse{
      let body = serde_json::to_string(&self).unwrap();

      // Create response and set content type
      HttpResponse::Ok().content_type("application/json").body(body)
  }
}


