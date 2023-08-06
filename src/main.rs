mod database;
mod blog;
extern crate dotenv;

use std::net::SocketAddr;
use axum::{ Router, response::IntoResponse, routing::{ get, patch, delete, post }, Server};
use blog::BlogCtrl;
use dotenv::dotenv;


#[tokio::main]
async fn main() {
    //setup dotenv
    dotenv().ok();
    //connectDb
    let db = database::connect_db().await.expect("Failed to connect to database");
    // run migrations
    sqlx::migrate!("./migrations").run(&db).await.expect("Failed to run migrations");

    // routes
    let all_routes = Router::new()
      .route("/", get(root))
      .route("/blog", get(BlogCtrl::get_all_blogs))
      .route("/blog", post(BlogCtrl::create_blog))
      .route("/blog/:id", delete(BlogCtrl::delete_blog))
      .route("/blog/:id", patch(BlogCtrl::update_blog))
      .with_state(db);

    let addr = SocketAddr::from(([0,0,0,0], 3001));
    println!("Server is running on port {addr}...");
    
    // server
    Server::bind(&addr).serve(all_routes.into_make_service()).await.unwrap();
}


async fn root() -> impl IntoResponse {
    "Hello world"
}
