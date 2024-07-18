use error::ResultInftra;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

pub mod error;
pub mod site;

pub async fn init_db(
    addr: String,
    username: String,
    password: String,
    namespace: String,
    database: String,
) -> ResultInftra<Surreal<Client>> {
    let db = Surreal::new::<Ws>(addr.as_str()).await?;

    db.signin(Root {
        username: username.as_str(),
        password: password.as_str(),
    })
    .await?;

    db.use_ns(namespace.as_str())
        .use_db(database.as_str())
        .await?;

    Ok(db)
}
