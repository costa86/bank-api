use crate::database::{crud, models};
use actix_web::{web, HttpRequest, HttpResponse, Responder, Result};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use validator::Validate;

static SECRET_KEY: &[u8; 6] = b"secret";
static TOKEN_EXPIRATION_MINUTES: u16 = 1 * 60 * 24;

pub async fn get_jwt() -> impl Responder {
    let key = SECRET_KEY;
    let iat = Utc::now().timestamp();

    let exp = Utc::now()
        .checked_add_signed(Duration::minutes(TOKEN_EXPIRATION_MINUTES.into()))
        .expect("Invalid exp")
        .timestamp();

    let claims = models::Claims {
        sub: "admin@mail.com".to_owned(),
        iat: iat as usize,
        exp: exp as usize,
        role: "admin".to_owned(),
    };

    let token = match encode(&Header::default(), &claims, &EncodingKey::from_secret(key)) {
        Ok(x) => x,
        Err(_) => panic!(),
    };

    HttpResponse::Ok().body(token)
}

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

fn validate_balance(amount: f64, customer: &models::Customer) -> bool {
    amount <= customer.balance.unwrap()
}

fn validate_transfer(
    customer_from: &models::Customer,
    customer_to: &models::Customer,
    amount: f64,
) -> bool {
    let updated_balance_customer_from = crud::update_balance(
        customer_from.id.unwrap(),
        customer_from.balance.unwrap() - amount,
    );
    let updated_balance_customer_to = crud::update_balance(
        customer_to.id.unwrap(),
        customer_to.balance.unwrap() + amount,
    );

    let transfer_created =
        crud::create_transfer(customer_from.id.unwrap(), customer_to.id.unwrap(), amount);

    updated_balance_customer_from.is_ok()
        && updated_balance_customer_to.is_ok()
        && transfer_created.is_ok()
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

    if !validate_balance(transfer.amount, &customer_from) {
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

pub async fn get_transfers_by_customer(id: web::Path<u16>) -> impl Responder {
    let mut response = models::APIResponse {
        message: "could not get customer".to_string(),
    };

    match crud::get_customer(*id) {
        Ok(x) => match crud::get_transfers_by_customer(x.id.unwrap()) {
            Ok(x) => HttpResponse::Ok().json(x),
            Err(_) => {
                response.message = "could not get transfers".to_string();
                HttpResponse::NotFound().json(response)
            }
        },
        Err(_) => HttpResponse::NotFound().json(response),
    }
}

pub async fn get_payments_by_customer(id: web::Path<u16>) -> impl Responder {
    let mut response = models::APIResponse {
        message: "could not get customer".to_string(),
    };

    match crud::get_customer(*id) {
        Ok(x) => match crud::get_payments_by_customer(x.id.unwrap()) {
            Ok(x) => HttpResponse::Ok().json(x),
            Err(_) => {
                response.message = "could not get payments".to_string();
                HttpResponse::NotFound().json(response)
            }
        },
        Err(_) => HttpResponse::NotFound().json(response),
    }
}

pub fn validate_token(req: HttpRequest) -> Option<models::Claims> {
    let key = SECRET_KEY;
    let token = req.headers().get("authorization");
    if token.is_none() {
        return None;
    }
    let token = token
        .unwrap()
        .to_str()
        .unwrap()
        .split_whitespace()
        .nth(1)
        .unwrap();

    match decode::<models::Claims>(
        &token,
        &DecodingKey::from_secret(key),
        &Validation::default(),
    ) {
        Ok(x) => Some(x.claims),
        Err(_) => None,
    }
}
pub async fn withdraw(money: web::Json<models::Money>, id: web::Path<u16>) -> impl Responder {
    let mut response = models::APIResponse {
        message: "could not withdraw".to_string(),
    };

    let validation = money.validate();
    if validation.is_err() {
        return HttpResponse::UnprocessableEntity().json(validation.err());
    }

    let customer_found = crud::get_customer(*id);

    match customer_found {
        Err(_) => {
            response.message = "could not find customer".to_string();
            return HttpResponse::NotFound().json(response);
        }
        Ok(x) => {
            if !validate_balance(money.amount, &x) {
                response.message = "not enough balance".to_string();
                return HttpResponse::BadRequest().json(response);
            }
            let balance = x.balance.unwrap() - money.amount;

            match crud::update_balance(x.id.unwrap(), balance) {
                Err(_) => return HttpResponse::BadRequest().json(response),
                Ok(_) => {
                    response.message = "withdrawal successfull".to_string();
                    return HttpResponse::Ok().json(response);
                }
            }
        }
    }
}

pub async fn deposit(money: web::Json<models::Money>, id: web::Path<u16>) -> impl Responder {
    let mut response = models::APIResponse {
        message: "could not deposit".to_string(),
    };

    let validation = money.validate();
    if validation.is_err() {
        return HttpResponse::UnprocessableEntity().json(validation.err());
    }

    let customer_found = crud::get_customer(*id);

    match customer_found {
        Err(_) => {
            response.message = "could not find customer".to_string();
            return HttpResponse::NotFound().json(response);
        }
        Ok(x) => {
            let balance = x.balance.unwrap() + money.amount;

            if crud::update_balance(x.id.unwrap(), balance).is_ok() {
                response.message = "deposit successfull".to_string();
                return HttpResponse::Ok().json(response);
            };
        }
    }
    HttpResponse::BadRequest().json(response)
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

pub async fn get_all_customers(_req: HttpRequest) -> impl Responder {
    // if validate_token(req).is_none() {
    //     return HttpResponse::BadRequest().json("Missing or invalid Token");
    // }

    let customer_list = crud::get_all_customers();

    match customer_list {
        Ok(x) => HttpResponse::Ok().json(x),
        Err(_e) => {
            let response = models::APIResponse {
                message: "could not get customers".to_string(),
            };
            HttpResponse::BadRequest().json(response)
        }
    }
}

pub async fn get_all_transfers() -> impl Responder {
    let record_list = crud::get_all_transfers();

    match record_list {
        Ok(x) => HttpResponse::Ok().json(x),
        Err(_e) => {
            let response = models::APIResponse {
                message: "could not get transfers".to_string(),
            };
            HttpResponse::BadRequest().json(response)
        }
    }
}

pub async fn create_customer(mut customer: web::Json<models::Customer>) -> Result<impl Responder> {
    let mut response = models::APIResponse {
        message: "customer not created".to_string(),
    };
    customer.created_at = Some(Utc::now().to_rfc2822());
    let created_customer = customer.into_inner();

    if created_customer.validate().is_err() {
        return Ok(HttpResponse::UnprocessableEntity().json(created_customer.validate().err()));
    }

    match crud::create_customer(&created_customer) {
        Ok(_x) => {
            response.message = "customer created".to_string();
            return Ok(HttpResponse::Ok().json(response));
        }
        Err(_e) => Ok(HttpResponse::BadRequest().json(response)),
    }
}

pub async fn create_payment(
    payment: web::Json<models::Payment>,
    id: web::Path<u16>,
) -> impl Responder {
    let mut response = models::APIResponse {
        message: "payment not created".to_string(),
    };
    let mut created_payment = payment.into_inner();

    if created_payment.validate().is_err() {
        return HttpResponse::UnprocessableEntity().json(created_payment.validate().err());
    }

    match crud::get_customer(*id) {
        Err(_) => {
            response.message = "could not find customer".to_string();
            return HttpResponse::NotFound().json(response);
        }
        Ok(x) => {
            if !validate_balance(created_payment.amount, &x) {
                response.message = "not enough balance".to_string();
                return HttpResponse::BadRequest().json(response);
            }
            let balance = x.balance.unwrap() - created_payment.amount;
            created_payment.created_at = Some(Utc::now().to_rfc2822());
            created_payment.customer_id = x.id;

            match crud::create_payment(&created_payment, balance) {
                Ok(_) => {
                    response.message = "payment successfull".to_string();
                    return HttpResponse::Ok().json(response);
                }
                Err(_) => return HttpResponse::BadRequest().json(response),
            }
        }
    }
}
