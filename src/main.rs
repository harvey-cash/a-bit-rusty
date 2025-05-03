use std::sync::{Arc, Mutex};

use axum::{
    extract::State, http::StatusCode, routing::get, Json, Router
};

use a_bit_rusty::chip::chip_database::*;

#[derive(Debug)]
struct AppState {
    database: ChipDatabase
}

type SharedState = Arc<Mutex<AppState>>;

#[tokio::main]
async fn main() {
    let initial_state = AppState {
        database: ChipDatabase::new()
    };
    let shared_state: SharedState = Arc::new(Mutex::new(initial_state));
    
    let app = Router::new()
        .route("/chip_list", get(get_chip_list))
        .with_state(shared_state);
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_chip_list(
    State(state): State<SharedState>
) -> Result<Json<Vec<ChipKey>>, StatusCode> {
    println!("->> HANDLE get_chip_list");
    let app_state = state.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(app_state.database.get_chip_list().clone()))
}