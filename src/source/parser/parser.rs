use crate::article::article;

pub trait Parser {
  fn parse_rss(&self, xml: &str) -> Vec<article::Article>;
}