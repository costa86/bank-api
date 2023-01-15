use crate::database::{crud, models};
use actix_web::{get, post, web, HttpResponse, Responder, Result};

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn get_customer(id: web::Path<u16>) -> impl Responder {
    let customer = crud::get_customer(*id);
    if customer.is_ok() {
        return HttpResponse::Ok().json(customer.unwrap());
    }
    let response = models::APIResponse {
        message: "could not get customer".to_string(),
    };
    HttpResponse::BadRequest().json(response)
}

pub async fn get_all_customers() -> impl Responder {
    let customer_list = crud::get_all_customers();

    if customer_list.is_ok() {
        return HttpResponse::Ok().json(customer_list.unwrap());
    }
    let response = models::APIResponse {
        message: "could not get customers".to_string(),
    };
    HttpResponse::BadRequest().json(response)
}

pub async fn create_customer(customer: web::Json<models::Customer>) -> Result<impl Responder> {
    let mut response = models::APIResponse {
        message: "customer created".to_string(),
    };

    if crud::create_customer(&customer).is_ok() {
        return Ok(HttpResponse::Ok().json(response));
    }
    response.message = "customer not created".to_string();
    Ok(HttpResponse::BadRequest().json(response))
}
