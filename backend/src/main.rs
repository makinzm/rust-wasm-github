use actix_web::{get, App, HttpServer, HttpResponse, Responder};
use mysql::*;
use mysql::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use actix_cors::Cors;

#[derive(Debug, Serialize, Deserialize)]
struct WordEntry {
    id: i32,
    priority: i32,
    word: String,
    meaning: String,
    learning_history: Value,
}

#[get("/words")]
async fn get_words() -> impl Responder {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    eprintln!("Connecting to {}", url);
    
    let opts = Opts::from_url(&url).unwrap();
    let pool = Pool::new(opts).unwrap();
    let mut conn = pool.get_conn().unwrap();

    let words: Vec<WordEntry> = conn
        .query_map(
            "SELECT * FROM WordEntry",
            |(id, priority, word, meaning, learning_history)| {
                WordEntry { id, priority, word, meaning, learning_history }
            },
        )
        .unwrap();

    HttpResponse::Ok().json(words)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
        App::new()
            .wrap(cors)
            .service(get_words)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

