use actix_files::Files;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    stuff::init();

    HttpServer::new(|| {
        App::new()
            .configure(stuff::routes)
            .service(Files::new("/", "./public/").index_file("index.html"))
    })
    .bind("0.0.0.0:80")?
    .bind("[::0]:80")?
    .run()
    .await
}