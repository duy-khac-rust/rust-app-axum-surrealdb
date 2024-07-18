use axum::Router;
use site::site_routes;
use surrealdb::{engine::remote::ws::Client, Surreal};

pub mod site;


pub fn routes() -> Router<Surreal<Client>> {
    Router::new().nest(
        "/api/v4", 
        Router::new()
            .merge(site_routes())
    )
}