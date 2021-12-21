use actix_web::{HttpResponse, HttpRequest};
use crate::{
  article::article
};
use std::collections::HashMap;
use crate::repository;
use serde_json;
use mongodb::bson::Bson;

pub async fn index(_req: HttpRequest) -> HttpResponse {
  let repository = repository::Repository::<article::Article>::new().await;
  let articles = repository.find(HashMap::<String, Bson>::new()).await;
  let body = serde_json::to_string(&articles).unwrap();

  HttpResponse::Ok().content_type("application/json").body(body)
}