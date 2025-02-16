use crate::models::validate::{CheckEmailOutputResponse, CheckEmailRequest};
use actix_web::web::Json;
use actix_web::{post, HttpResponse, Responder};
use check_if_email_exists::{check_email, CheckEmailInput, Reachable};

pub fn validate_routes(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(check_email_exists);
}

#[post("/check-email")]
async fn check_email_exists(body: Json<CheckEmailRequest>) -> impl Responder {
    println!("Checking email: {:?}", body.email);
    let input = CheckEmailInput::new(body.email.clone());

    let result = check_email(&input).await;

    println!("Is reachable: {:?}", result.is_reachable);

    // Convert the Reachable enum to a boolean.
    let is_reachable_bool = match result.is_reachable {
        Reachable::Risky => true,
        Reachable::Safe => true,
        _ => false,
    };

    let response = CheckEmailOutputResponse {
        input: body.email.clone(),
        is_reachable: is_reachable_bool,
        reachable: result.is_reachable.to_string(),
    };

    HttpResponse::Ok().json(response)
}
