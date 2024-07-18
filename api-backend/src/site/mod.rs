use about::get_about_routes;
use axum::Router;
use root::get_root_routes;

use crate::SC;

pub mod about;
pub mod root;

pub fn site_routes() -> Router<SC> {
    Router::new()
        .merge(get_root_routes())
        .merge(get_about_routes())
}
