use crate::{connect, set};
use actix_web::{web, HttpResponse};
use rand::{distributions::Alphanumeric, Rng};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SignupRequestBody {
    username: String,
}

pub async fn signup(body: web::Json<SignupRequestBody>) -> HttpResponse {
    if body.username.is_empty() {
        return HttpResponse::BadRequest()
            .content_type("application/json")
            .body("no username was passed in the request body");
    }
    let api_key: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(12)
        .map(char::from)
        .collect();

    set(&mut connect(), api_key.as_str(), &body.username);

    HttpResponse::Ok()
        .content_type("application/json")
        .body(api_key)
}
