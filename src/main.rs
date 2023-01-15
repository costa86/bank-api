mod database;
mod routes;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // database::crud::create_db().unwrap();

    let host = "127.0.0.1";
    let port = 8080;

    HttpServer::new(|| {
        App::new()
            .service(routes::hello)
            .service(routes::echo)
            .service(
                web::scope("/customers")
                    .route("", web::post().to(routes::create_customer))
                    .route("", web::get().to(routes::get_all_customers))
                    .route("/{id}", web::get().to(routes::get_customer))
                    .route("/{id}", web::put().to(routes::edit_customer))
                    .route("/transfer/", web::put().to(routes::transfer_amount)),
            )
            .route("/hey", web::get().to(manual_hello))
    })
    .bind((host, port))?
    .run()
    .await
}
