use crate::database::{crud, models};
use actix_web::{get, post, web, HttpResponse, Responder, Result};
use validator::Validate;

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

fn validate_balance(transfer: &web::Json<models::Transfer>, customer: &models::Customer) -> bool {
    transfer.amount < customer.balance.unwrap()
}

fn validate_transfer(
    customer_from: &models::Customer,
    customer_to: &models::Customer,
    amount: f64,
) -> bool {
    let update_balance_customer_from = crud::update_balance(
        customer_from.id.unwrap(),
        customer_from.balance.unwrap() - amount,
    );
    let update_balance_customer_to = crud::update_balance(
        customer_to.id.unwrap(),
        customer_to.balance.unwrap() + amount,
    );
    update_balance_customer_from.is_ok() && update_balance_customer_to.is_ok()
}

pub async fn transfer_amount(transfer: web::Json<models::Transfer>) -> impl Responder {
    let mut response = models::APIResponse {
        message: "could not process the transfer".to_string(),
    };

    let validation = transfer.validate();
    if validation.is_err() {
        return HttpResponse::UnprocessableEntity().json(validation.err());
    }
    if transfer.id_from == transfer.id_to {
        response.message = "cannot transfer from and to the same customer".to_string();
        return HttpResponse::UnprocessableEntity().json(response);
    }

    let customer_from = crud::get_customer(transfer.id_from.into());

    if customer_from.is_err() {
        response.message = "could not find customer to transfer from".to_string();
        return HttpResponse::NotFound().json(response);
    }

    let customer_from = customer_from.unwrap();

    if !validate_balance(&transfer, &customer_from) {
        response.message = "not enough balance".to_string();
        return HttpResponse::BadRequest().json(response);
    }

    let customer_to = crud::get_customer(transfer.id_to.into());

    if customer_to.is_err() {
        response.message = "could not find customer to tranfer to".to_string();
        return HttpResponse::NotFound().json(response);
    }

    if validate_transfer(&customer_from, &customer_to.unwrap(), transfer.amount) {
        response.message = "transfer successfull".to_string();
        return HttpResponse::Ok().json(response);
    }
    HttpResponse::BadRequest().json(response)
}

pub async fn get_customer(id: web::Path<u16>) -> impl Responder {
    let response = models::APIResponse {
        message: "could not get customer".to_string(),
    };

    match crud::get_customer(*id) {
        Ok(x) => HttpResponse::Ok().json(x),
        Err(_e) => HttpResponse::NotFound().json(response),
    }
}

pub async fn edit_customer(
    customer: web::Json<models::CustomerEdit>,
    id: web::Path<u16>,
) -> impl Responder {
    let mut response = models::APIResponse {
        message: "could not edit customer".to_string(),
    };

    let customer_found = crud::get_customer(*id);

    if customer_found.is_err() {
        response.message = "could not find customer".to_string();
        return HttpResponse::NotFound().json(response);
    }

    let validation = customer.validate();
    if validation.is_err() {
        return HttpResponse::UnprocessableEntity().json(validation.err());
    }

    if crud::edit_customer(*id, customer.into_inner()).is_ok() {
        response.message = "customer edited".to_string();
        return HttpResponse::Ok().json(response);
    }

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
        message: "customer not created".to_string(),
    };

    let created_customer = customer.into_inner();

    if created_customer.validate().is_err() {
        return Ok(HttpResponse::UnprocessableEntity().json(created_customer.validate().err()));
    }

    if crud::create_customer(&created_customer).is_ok() {
        response.message = "customer created".to_string();

        return Ok(HttpResponse::Ok().json(response));
    }
    return Ok(HttpResponse::BadRequest().json(response));
}
