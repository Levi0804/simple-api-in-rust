use std::fs;
use lazy_static::lazy_static;
use actix_web::{web, Responder, get, HttpResponse};
use crate::{
    model::*,
    response::*
};

lazy_static! {
    pub static ref GROUPS_JSON: String = fs::read_to_string("data/all_groups.json").unwrap();
}

#[get("/groups")]
pub async fn get_groups(query: web::Query<QueryParams>) -> impl Responder {
    let groups: Vec<Group> = serde_json::from_str(GROUPS_JSON.as_str()).unwrap();

    match &query.q {
        Some(q) => {
            let group = groups
                .into_iter()
                .find(|group| group.name == q);

            if let Some(group) = group {
                HttpResponse::Ok().json(group)
            } else {
                HttpResponse::BadRequest().json(NotFound {
                    status: "failed",
                    message: "Invalid name"
                })
            }
        },
        None => {
            let length = groups.len();
            HttpResponse::Ok().json(Groups { length, groups })
        }
    }
}

#[get("/groups/{id}")]
async fn get_group(path: web::Path<u32>) -> HttpResponse {
    let id = path.into_inner();

    let groups: Vec<Group> = serde_json::from_str(&GROUPS_JSON.as_str()).unwrap();
    let group = groups
        .into_iter()
        .find(|group| group.id == id);

    match group {
        Some(group) => HttpResponse::Ok().json(group),
        None => HttpResponse::BadRequest().json(NotFound {
            status: "failed",
            message: "Invalid ID"
        })
    }
}
