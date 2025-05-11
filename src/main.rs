use a_bit_rusty::chip::designer::{ChipPinLink, Designer, DesignerState};
use axum::{
    body::Body, extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    }, http::{Response, StatusCode}, response::{Html, IntoResponse}, routing::{get, post}, Json, Router
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    fs, net::SocketAddr, sync::{Arc, Mutex}
};
use tokio::net::TcpListener;

struct AppState {
    designer: Designer,
}

type SharedState = Arc<Mutex<AppState>>;

async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<SharedState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: SharedState) {
    println!("Client connected. Initializing session.");
    while let Some(Ok(msg)) = socket.recv().await {
        match msg {
            Message::Text(text) => {
                if text == "tick" {
                    let designer_state: DesignerState;
                    {
                        let mut app_state = state.lock().unwrap();
                        app_state.designer.tick();
                        designer_state = app_state.designer.get_state();
                    }

                    let state_json = json!({
                        "designer": designer_state,
                    });

                    if socket
                        .send(Message::Text(state_json.to_string().into()))
                        .await
                        .is_err()
                    {
                        println!("Client disconnected or error sending. Ending session.");
                        return;
                    }
                } else {
                    println!("Received unexpected text message: {}", text);
                }
            }
            Message::Binary(bin) => {
                println!("Received binary message: {} bytes", bin.len());
            }
            Message::Ping(ping_data) => {
                println!("Received ping");
                if socket.send(Message::Pong(ping_data)).await.is_err() {
                    println!("Failed to send pong. Client might be disconnected.");
                    return;
                }
            }
            Message::Pong(_) => {
                println!("Received pong");
            }
            Message::Close(close_frame) => {
                if let Some(cf) = close_frame {
                    println!(
                        "Client sent close message: code={}, reason='{}'",
                        cf.code, cf.reason
                    );
                } else {
                    println!("Client sent close message without a frame.");
                }
                return;
            }
        }
    }
    println!("Client session ended (socket.recv() returned None or an error not caught above).");
}

async fn get_root_handler() -> impl IntoResponse {
    let file_contents = fs::read_to_string("client.html");
    match file_contents {
        Ok(html_content) => Html(html_content).into_response(),
        Err(e) => {
            eprintln!("Failed to read client.html: {}", e);
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(format!("Failed to load client.html: {}", e)))
                .unwrap()
        }
    }
}

#[derive(Serialize)]
struct IdResponse { id: usize }

#[derive(Serialize, Deserialize)]
struct IdValue { id: usize, value: u8 }

async fn add_chip_handler(
    State(state): State<SharedState>,
    Json(key): Json<String>,
) -> impl IntoResponse {
    let mut app_state = state.lock().unwrap();
    app_state.designer.add_chip(&key)
        .map(|id| (StatusCode::OK, Json(IdResponse { id: id })))
        .map_err(|message| (StatusCode::INTERNAL_SERVER_ERROR, Json(message)))
}

async fn add_link_handler(
    State(state): State<SharedState>,
    Json(link): Json<ChipPinLink>,
) -> impl IntoResponse {
    let mut app_state = state.lock().unwrap();
    app_state.designer.add_link(link)
        .map(|_| (StatusCode::OK, Json(0)))
        .map_err(|message| (StatusCode::INTERNAL_SERVER_ERROR, Json(message)))
}

async fn set_input_value_handler(
    State(state): State<SharedState>,
    Json(id_value): Json<IdValue>,
) -> impl IntoResponse {
    let mut app_state = state.lock().unwrap();
    app_state.designer.set_input_chip_value(id_value.id, id_value.value)
        .map(|_| (StatusCode::OK, Json(0)))
        .map_err(|message| (StatusCode::INTERNAL_SERVER_ERROR, Json(message)))
}

#[tokio::main]
async fn main() {
    let shared_state = Arc::new(Mutex::new(AppState {
        designer: Designer::new(),
    }));

    let app = Router::new()
        .route("/", get(get_root_handler))
        .route("/designer/add_chip", post(add_chip_handler))
        .route("/designer/add_link", post(add_link_handler))
        .route("/designer/set_input_value", post(set_input_value_handler))
        .route("/ws/designer", get(websocket_handler))
        .with_state(shared_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server listening on {}", addr);

    let listener = match TcpListener::bind(&addr).await {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Failed to bind to address {}: {}", addr, e);
            return;
        }
    };

    if let Err(e) = axum::serve(listener, app.into_make_service()).await {
        eprintln!("Server error: {}", e);
    }
}