use super::error::ResultSite;


pub async fn view_root() -> ResultSite<&'static str> {
    Ok("Welcome to root page!")
}