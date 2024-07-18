use api_backend::routes;
use core_backend::ProdConfig;
use dotenv::dotenv;
use inftra_backend::init_db;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    // Config file env
    let cfg = match ProdConfig::from_env().await {
        Ok(env) => env,
        Err(err) => {
            panic!("Error loading config: {}", err)
        }
    };

    // Connect to database
    let db = match init_db(
        cfg.surrealdb.addr,
        cfg.surrealdb.username,
        cfg.surrealdb.password,
        cfg.surrealdb.namespace,
        cfg.surrealdb.database,
    )
    .await
    {
        Ok(d) => d,
        Err(e) => panic!("Error connecting to database: {}", e),
    };

    // Start server
    let app = routes().with_state(db);

    let listener = TcpListener::bind(cfg.web.addr.as_str())
        .await
        .expect("listener is failed!");

    axum::serve(listener, app).await.unwrap();
}
