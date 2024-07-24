use std::fs;
use lazy_static::lazy_static;
use actix_web::{web, Responder, get, HttpResponse};
use crate::{
    model::*,
    response::*,
};

lazy_static! {
    static ref ARTISTS_JSON: String = fs::read_to_string("data/all_artists.json").unwrap();
    static ref ARTISTS: Vec<Artist<'static>> = serde_json::from_str(ARTISTS_JSON.as_str()).unwrap();
}

#[get("/artists")]
pub async fn get_artists(query: web::Query<QueryParams>) -> impl Responder {
    if let Some(ref q) = query.q {
        let artist = ARTISTS
            .to_vec()
            .into_iter()
            .find(|artist| artist.full_name == q);
        
        artist.map_or(HttpResponse::BadRequest().json(NotFound {
            status: "failed",
            message: "Invalid name"
        }), |artist| HttpResponse::Ok().json(artist))
    } else {
        HttpResponse::Ok().json(Artists { 
            length: ARTISTS.len(),
            artists: ARTISTS.to_vec() 
        })
    }
}

#[get("/artists/{id}")]
async fn get_artist(path: web::Path<u32>) -> HttpResponse {
    let id = path.into_inner();
    let artist = ARTISTS
        .to_vec()
        .into_iter()
        .find(|artist| artist.id == id);

    artist.map_or(HttpResponse::BadRequest().json(NotFound {
        status: "failed",
        message: "Invalid name"
    }), |artist| HttpResponse::Ok().json(artist))
}
