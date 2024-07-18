
use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};


pub async fn mw_auth(req: Request, next: Next) -> Response {
    println!("--->> MIDDLEWARE AUTH ");

    next.run(req).await
}