require 'mongo'
require_relative './environment'

sources = [
  {
    id: 1,
    code: "rbc",
    name: "RBC news",
    rss_feed_url: "https://rssexport.rbc.ru/rbcnews/news/30/full.rss",
    parser_kind: 1
  },
  {
    id: 2,
    code: "hacker_news",
    name: "Hacker News",
    rss_feed_url: "https://news.ycombinator.com/rss",
    parser_kind: 2
  },
  {
    id: 3,
    code: "medium_technology",
    name: "Medium Technology",
    rss_feed_url: "https://medium.com/feed/tag/technology",
    parser_kind: 3 
  }
]


client = Mongo::Client.new("#{ENV['MONGODB_URI']}/news_aggregator")

client[:sources].insert_many(sources)
