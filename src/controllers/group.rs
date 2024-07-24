use std::fs;
use lazy_static::lazy_static;
use actix_web::{web, Responder, get, HttpResponse};
use crate::{
    model::*,
    response::*
};

lazy_static! {
    static ref GROUPS_JSON: String = fs::read_to_string("data/all_groups.json").unwrap();
    static ref GROUPS: Vec<Group<'static>> = serde_json::from_str(GROUPS_JSON.as_str()).unwrap();
}

#[get("/groups")]
pub async fn get_groups(query: web::Query<QueryParams>) -> impl Responder {
    if let Some(ref q) = query.q {
        let group = GROUPS
            .to_vec()
            .into_iter()
            .find(|group| group.name == q);
        
        group.map_or(HttpResponse::BadRequest().json(NotFound {
            status: "failed",
            message: "Invalid name"
        }), |group|HttpResponse::Ok().json(group))
    } else {
        HttpResponse::Ok().json(Groups {
            length: GROUPS.len(),
            groups: GROUPS.to_vec() 
        })
    }
}

#[get("/groups/{id}")]
async fn get_group(path: web::Path<u32>) -> HttpResponse {
    let id = path.into_inner();
    let group = GROUPS
        .to_vec()
        .into_iter()
        .find(|group| group.id == id);

    group.map_or(HttpResponse::BadRequest().json(NotFound {
        status: "failed",
        message: "Invalid name"
    }), |group|HttpResponse::Ok().json(group))
}
