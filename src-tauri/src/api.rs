use std::net::SocketAddr;
use axum::routing::{get, post};
use axum::Router;
use tokio::net::TcpListener;
use crate::controllers::user_controller;

pub fn start() {
    let api: Router<()> = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/create_user", post(user_controller::create_user))
        .route("/list_users", get(user_controller::list_users))
        .route("/user", get(user_controller::get_user))
        .with_state(());

    tokio::spawn(async move {
        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

        println!("Starting server on {}", addr);

        match axum::serve(TcpListener::bind(addr).await.unwrap(), api).await {
            Ok(_) => (),
            Err(e) => eprintln!("Error starting server: {}", e),
        }
    });
}