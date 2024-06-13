use crate::controllers::{healthcheck, actor, artist, group};
use actix_web::web;

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(healthcheck::health_checker_handler)
        .service(actor::get_actors)
        .service(actor::get_actor)
        .service(artist::get_artists)
        .service(artist::get_artist)
        .service(group::get_groups)
        .service(group::get_group);
    conf.service(scope);
}
