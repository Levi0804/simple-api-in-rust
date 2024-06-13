use std::fs;
use lazy_static::lazy_static;
use actix_web::{web, Responder, get, HttpResponse};
use crate::{
    model::*,
    response::*,
};

lazy_static! {
    pub static ref ARTISTS_JSON: String = fs::read_to_string("data/all_artists.json").unwrap();
}

#[get("/artists")]
pub async fn get_artists(query: web::Query<QueryParams>) -> impl Responder {
    let artists: Vec<Artist> = serde_json::from_str(ARTISTS_JSON.as_str()).unwrap();

    match &query.q {
        Some(q) => {
            let artist = artists
                .into_iter()
                .find(|artist| artist.full_name == q);

            if let Some(artist) = artist {
                HttpResponse::Ok().json(artist)
            } else {
                HttpResponse::BadRequest().json(NotFound {
                    status: "failed",
                    message: "Invalid name"
                })
            }
        },
        None => {
            let length = artists.len();
            HttpResponse::Ok().json(Artists { length, artists })
        }
    }
}

#[get("/artists/{id}")]
async fn get_artist(path: web::Path<u32>) -> HttpResponse {
    let id = path.into_inner();

    let artists: Vec<Artist> = serde_json::from_str(&ARTISTS_JSON.as_str()).unwrap();
    let artist = artists
        .into_iter()
        .find(|artist| artist.id == id);

    match artist {
        Some(artist) => HttpResponse::Ok().json(artist),
        None => HttpResponse::BadRequest().json(NotFound {
            status: "failed",
            message: "Invalid ID"
        })
    }
}
