use crate::controllers::{auth_controller, book_controller, user_controller};
use axum::routing::{get, post};
use axum::Router;
use std::net::SocketAddr;
use tokio::net::TcpListener;

pub fn start() {
    let api: Router<()> = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/register", post(user_controller::create_user))
        .route("/list_users", get(user_controller::list_users))
        .route("/user", get(user_controller::get_user))
        .route("/login", post(auth_controller::login))
        .route("/refresh", post(auth_controller::refresh))
        .route("/logout", post(auth_controller::logout))
        .route("/book/:id/content", get(book_controller::get_book_content))
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
