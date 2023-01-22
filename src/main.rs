mod database;
mod routes;
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    database::crud::check_db().unwrap();

    let host = "127.0.0.1";
    let port = 8080;

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(routes::health_check))
            .route("token", web::get().to(routes::get_jwt))
            .service(
                web::scope("/customers")
                    .route("", web::post().to(routes::create_customer))
                    .route("", web::get().to(routes::get_all_customers))
                    .route("/{id}", web::get().to(routes::get_customer))
                    .route("/{id}", web::put().to(routes::edit_customer))
                    .route("/{id}/deposit", web::put().to(routes::deposit))
                    .route("/{id}/withdrawal", web::put().to(routes::withdraw))
                    .route("/transfer/", web::put().to(routes::transfer_amount)),
            )
            .service(web::scope("/transfers").route("", web::get().to(routes::get_all_transfers)))
    })
    .bind((host, port))?
    .run()
    .await
}
