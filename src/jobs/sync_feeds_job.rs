use crate::{
  jobs::Job,
  source::source::Source,
  repository::Repository
};
use tokio::{ task, time };
use std::collections::HashMap;
use mongodb::bson::Bson;

pub struct SyncFeedsJob();

impl Job for SyncFeedsJob {
  fn perform() {
    task::spawn(async {
      let mut interval = time::interval(time::Duration::from_secs(60));
      let mut sources = Repository::<Source>::new()
        .await
        .find(HashMap::<String, Bson>::new())
        .await;
   });
  }
}