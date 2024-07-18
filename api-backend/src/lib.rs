use axum::Router;
use site::site_routes;
use student::student_route;
use surrealdb::{engine::remote::ws::Client, Surreal};

pub type SC = Surreal<Client>;

// mod
pub mod site;
pub mod student;

pub fn routes() -> Router<SC> {
    Router::new().nest(
        "/api/v4", 
        Router::new()
            .merge(site_routes())
            .nest("/student", student_route())
    )
}