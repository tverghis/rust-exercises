use std::sync::{Arc, Mutex};

use outro_08::{router, AppState};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(AppState::new()));
    let app = router(state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on port 3000!");
    axum::serve(listener, app).await.unwrap();
}
