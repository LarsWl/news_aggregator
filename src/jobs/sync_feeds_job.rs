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
      let sources = Repository::<Source>::new()
        .await
        .find(HashMap::<String, Bson>::new())
        .await;

      loop {
        interval.tick().await;
        
        let sync_requests: Vec<_> = sources.iter().map(|source| source.sync_articles()).collect();

        for sync_request in sync_requests {
          sync_request.await;
        }
      }
   });
  }
}