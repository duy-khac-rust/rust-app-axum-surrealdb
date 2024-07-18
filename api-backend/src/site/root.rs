use axum::{routing::get, Router};
use inftra_backend::site::root::view_root;
use surrealdb::{engine::remote::ws::Client, Surreal};

pub fn get_root_routes() -> Router<Surreal<Client>> {
    pub async fn get_root() -> &'static str {
        view_root().await.unwrap()
    }

    Router::new().route("/", get(get_root))
}
