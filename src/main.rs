use a_bit_rusty::chip::designer::Designer;
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use serde_json::json;
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
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
                    let designer_state;
                    {
                        let mut app_state = state.lock().unwrap();
                        let _ = app_state.designer.tick();
                        designer_state = app_state.designer.get_state();
                    }

                    let response = json!({
                        "designer": designer_state,
                    });

                    if socket
                        .send(Message::Text(response.to_string().into()))
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

#[tokio::main]
async fn main() {
    let shared_state = Arc::new(Mutex::new(AppState {
        designer: Designer::new(),
    }));

    let app = Router::new()
        .route("/ws", get(websocket_handler))
        .with_state(shared_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("WebSocket server with state listening on ws://{}", addr);

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