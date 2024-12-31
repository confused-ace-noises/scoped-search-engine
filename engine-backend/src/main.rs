// use std::error::Error;

// use actix_web::{get, post, web, App, HttpServer, Responder};
// use engine_backend::search_any;
// use sort_results::matches::Page;
// #[actix_web::main]
// pub async fn main() -> Result<(), Box<dyn Error>> {
//     HttpServer::new(|| {
//         App::new().service((search_any, test))
//     })
//     .bind(("127.0.0.1", 6728))?  // Bind to the appropriate address
//     .run()
//     .await?;
    
//     Ok(())
// }

// #[get("/{name}")]
// pub async fn test(name: web::Path<String>) -> impl Responder {
//     format!("Hello {}", name)
// }

use std::default;

use engine_backend::search_any;
use rocket::{http::Method, Config};
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    let cors = CorsOptions::default()
    .allowed_origins(AllowedOrigins::all())
    .allowed_methods(
        vec![Method::Get, Method::Post, Method::Patch]
            .into_iter()
            .map(From::from)
            .collect(),
    )
    .allow_credentials(true).to_cors().unwrap();

    rocket::build().configure(Config::figment().merge(("port", 6728))).mount("/", routes![index, search_any]).attach(cors)

}
