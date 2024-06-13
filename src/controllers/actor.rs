use lazy_static::lazy_static;
use std::fs;
use actix_web::{Responder, web, get, HttpResponse};
use crate::{
    response::*,
    model::*
};

lazy_static! {
    pub static ref ACTORS_JSON: String = fs::read_to_string("data/all_actors.json").unwrap();
}

#[get("/actors")]
pub async fn get_actors(query: web::Query<QueryParams>) -> impl Responder {
    let actors: Vec<Actor> = serde_json::from_str(ACTORS_JSON.as_str()).unwrap();

    match &query.q {
        Some(q) => {
            let actor = actors
                .into_iter()
                .find(|actor| actor.full_name == q);

            if let Some(actor) = actor {
                HttpResponse::Ok().json(actor)
            } else {
                HttpResponse::BadRequest().json(NotFound {
                    status: "failed",
                    message: "Invalid name"
                })
            }
        },
        None => {
            let length = actors.len();
            HttpResponse::Ok().json(Actors { length, actors })
        }
    }
}

#[get("/actors/{id}")]
async fn get_actor(path: web::Path<u32>) -> HttpResponse {
    let id = path.into_inner();

    let actors: Vec<Actor> = serde_json::from_str(&ACTORS_JSON.as_str()).unwrap();

    let actor = actors
        .into_iter()
        .find(|actor| actor.id == id);

    match actor {
        Some(actor) => HttpResponse::Ok().json(actor),
        None => HttpResponse::BadRequest().json(NotFound {
            status: "failed",
            message: "Invalid ID"
        })
    }
}
