use serde::{Deserialize, Serialize};
use crate::model::{Actor, Group, Artist};

#[derive(Serialize)]
pub struct Groups<'a> {
    pub groups: Vec<Group<'a>>,
    pub length: usize
}

#[derive(Serialize)]
pub struct Actors<'a> {
    pub actors: Vec<Actor<'a>>,
    pub length: usize
}

#[derive(Serialize)]
pub struct Artists<'a> {
    pub artists: Vec<Artist<'a>>,
    pub length: usize
}

#[derive(Serialize)]
pub struct GenericResponse<'a> {
    pub status: &'a str,
    pub message: &'a str,
}

#[derive(Serialize)]
pub struct NotFound<'a> {
    pub status: &'a str,
    pub message: &'a str 
}

#[derive(Serialize, Deserialize)]
pub struct QueryParams {
    pub q: Option<String>,
}
