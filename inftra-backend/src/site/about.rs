use super::error::ResultSite;

pub async fn views_about() -> ResultSite<&'static str> {
    Ok("Welcome to about page")
}
