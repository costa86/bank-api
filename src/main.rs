mod database;
mod routes;
use actix_web::{web, App, HttpServer};

static HOST: &str = "127.0.0.1";
static PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    database::crud::check_db().unwrap();

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(routes::health_check))
            .route("token", web::get().to(routes::get_jwt))
            .service(web::scope("/transfers").route("", web::get().to(routes::get_all_transfers)))
            .service(
                web::scope("/customers")
                    .route("/transfers", web::put().to(routes::transfer_amount))
                    .route("", web::post().to(routes::create_customer))
                    .route("", web::get().to(routes::get_all_customers))
                    .route("/{id}", web::get().to(routes::get_customer))
                    .route("/{id}", web::put().to(routes::edit_customer))
                    .route(
                        "/{id}/transfers",
                        web::get().to(routes::get_transfers_by_customer),
                    )
                    .route("/{id}/payments", web::post().to(routes::create_payment))
                    .route(
                        "/{id}/payments",
                        web::get().to(routes::get_payments_by_customer),
                    )
                    .route("/{id}/deposits", web::put().to(routes::deposit))
                    .route("/{id}/withdrawals", web::put().to(routes::withdraw)),
            )
    })
    .bind((HOST, PORT))?
    .run()
    .await
}
