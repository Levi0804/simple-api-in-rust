use crate::response::GenericResponse;
use actix_web::{get, Responder, HttpResponse};

#[get("/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "A simple kpop api written in rust lang";

    let response_json = &GenericResponse {
        status: "success",
        message: MESSAGE,
    };
    HttpResponse::Ok().json(response_json)
}
