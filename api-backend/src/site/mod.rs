use axum::Router;
use root::get_root_routes;
use surrealdb::{engine::remote::ws::Client, Surreal};

pub mod root;

pub fn site_routes() -> Router<Surreal<Client>> {
    Router::new()
        .merge(get_root_routes())
}
