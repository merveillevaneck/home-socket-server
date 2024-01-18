use axum::routing::get;
use socketioxide::{extract::SocketRef, SocketIo};

use tracing::info;
use tracing_subscriber::FmtSubscriber;

fn on_connect(socket: SocketRef) {
    info!("Socket:io connected: {:?} {:?}", socket.ns(), socket.id);
    println!("connected");

    socket.emit("message", "alert");
}

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder().finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let (layer, io) = SocketIo::new_layer();

    io.ns("/", on_connect);

    let app = axum::Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(layer);

    info!("starting server on port 3001");
    axum::Server::bind(&"127.0.0.1:3001".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
