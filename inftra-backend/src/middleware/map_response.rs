use uuid::Uuid;
use axum::{
    http::{Method, StatusCode, Uri},
    response::{IntoResponse, Response},
};


pub async fn mw_map_response(uri: Uri, res_method: Method, res: Response) -> Response {
    let uuid = Uuid::new_v4();
    println!("Uuid: {}", uuid.to_string());
    println!("{:#?}", uri.to_string());
    println!("{:#?}", res_method);

    (StatusCode::ACCEPTED, res).into_response()
}
