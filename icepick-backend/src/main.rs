use backend::
    server::server_utils::search
;
use rocket::{http::Method, launch, routes, Config};
use rocket_cors::{AllowedOrigins, CorsOptions};

#[launch]
async fn rocket() -> _ {
    let cors = CorsOptions::default()
    .allowed_origins(AllowedOrigins::all())
    .allowed_methods(
        vec![Method::Get, Method::Post, Method::Patch]
            .into_iter()
            .map(From::from)
            .collect(),
    )
    .allow_credentials(true).to_cors().unwrap();

    rocket::build().configure(Config::figment().merge(("port", 6728))).mount("/", routes![search]).attach(cors)
}