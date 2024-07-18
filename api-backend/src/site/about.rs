use axum::{routing::get, Router};
use inftra_backend::site::about::views_about;
use surrealdb::{engine::remote::ws::Client, Surreal};



pub fn get_about_routes() -> Router<Surreal<Client>> {
    pub async fn get_about() -> &'static str {
        views_about().await.unwrap()
    }

    Router::new().route("/about", get(get_about))
}
