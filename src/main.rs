mod handler;
mod model;
mod response;

use model::{QueryOptions};
use warp::{http::Method, Filter, Rejection};
use mongodb::{options::ClientOptions, Client, Database};
use std::sync::Arc;

type WebResult<T> = std::result::Result<T, Rejection>;
type MongoDB = Arc<Database>;

#[tokio::main]
async fn main() {
    // Set the log level for the app
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "api=info");
    }
    pretty_env_logger::init();

    // Connect to MongoDB
    let db = init_db().await.expect("Failed to initialize MongoDB");

    // Define routes
    let todo_router = warp::path!("api" / "todos");
    let todo_router_id = warp::path!("api" / "todos" / String);

    let health_checker = warp::path!("api" / "healthchecker")
        .and(warp::get())
        .and_then(handler::health_checker_handler);

    let cors = warp::cors()
        .allow_methods(&[Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_origins(vec!["http://localhost:3000/", "http://localhost:8000/"])
        .allow_headers(vec!["content-type"])
        .allow_credentials(true);

    let todo_routes = todo_router
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(handler::create_todo_handler)
        .or(todo_router
            .and(warp::get())
            .and(warp::query::<QueryOptions>())
            .and(with_db(db.clone()))
            .and_then(handler::todos_list_handler));

    let todo_routes_id = todo_router_id
        .and(warp::patch())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(handler::edit_todo_handler)
        .or(todo_router_id
            .and(warp::get())
            .and(with_db(db.clone()))
            .and_then(handler::get_todo_handler))
        .or(todo_router_id
            .and(warp::delete())
            .and(with_db(db.clone()))
            .and_then(handler::delete_todo_handler));

    let routes = todo_routes
        .with(cors)
        .with(warp::log("api"))
        .or(todo_routes_id)
        .or(health_checker);

    println!("🚀 Server started successfully");
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}

// Function to connect to MongoDB
async fn init_db() -> Result<MongoDB, mongodb::error::Error> {
    // Define your MongoDB connection string (local or remote)
    let client_uri = "mongodb://mongodb:27017"; // You can modify this if needed

    // Create a client and establish a connection to the database
    let mut client_options = ClientOptions::parse(client_uri).await?;
    client_options.app_name = Some("TodoApp".to_string());

    let client = Client::with_options(client_options)?;
    let db = client.database("todo_db");

    Ok(Arc::new(db))
}

// Filter for passing the DB handle to handlers
fn with_db(db: MongoDB) -> impl Filter<Extract = (MongoDB,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}