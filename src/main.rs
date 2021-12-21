mod article;
mod source;
mod db;
mod http_client;
mod repository;
mod handlers;
mod model;
mod jobs;
mod rss;

use crate::{
    handlers::*,
    jobs::*
};

use actix_web::{ get, web, App, HttpResponse, HttpServer, Responder };

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::from_filename(".env.local").ok();
    env_logger::init();

    sync_feeds_job::SyncFeedsJob::perform();

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .route("/articles", web::get().to(article_handler::index))
            .route("/sources", web::get().to(source_handler::index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}