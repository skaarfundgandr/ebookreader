// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::net::SocketAddr;

use axum::routing::{get, post};
use axum::Router;
use stellaron_lib::commands::sample;
use stellaron_lib::controllers::user_controller;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
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

    sample::run();
}
