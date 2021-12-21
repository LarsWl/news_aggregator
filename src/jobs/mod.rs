pub mod sync_feeds_job;

pub trait Job {
  fn perform();
} 