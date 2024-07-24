use lazy_static::lazy_static;
use std::fs;
use actix_web::{Responder, web, get, HttpResponse};
use crate::{
    response::*,
    model::*
};

lazy_static! {
    static ref ACTORS_JSON: String = fs::read_to_string("data/all_groups.json").unwrap();
    static ref ACTORS: Vec<Actor<'static>> = serde_json::from_str(&ACTORS_JSON).unwrap();
}

#[get("/actors")]
pub async fn get_actors(query: web::Query<QueryParams>) -> impl Responder {
    if let Some(ref q) = query.q {
        let actor = ACTORS
            .to_vec()
            .into_iter()
            .find(|actor| actor.full_name == q);

        actor.map_or(HttpResponse::BadRequest().json(NotFound {
            status: "failed",
            message: "Invalid name"
        }), |actor|HttpResponse::Ok().json(actor))
    } else {
        HttpResponse::Ok().json(Actors {
            length: ACTORS.len(),
            actors: ACTORS.to_vec() 
        })
    }
}

#[get("/actors/{id}")]
async fn get_actor(path: web::Path<u32>) -> HttpResponse {
    let id = path.into_inner();
    let actor = ACTORS
        .to_vec()
        .into_iter()
        .find(|actor| actor.id == id);

    actor.map_or(HttpResponse::BadRequest().json(NotFound {
        status: "failed",
        message: "Invalid name"
    }), |actor|HttpResponse::Ok().json(actor))
}
