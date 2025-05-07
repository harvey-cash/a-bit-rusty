use a_bit_rusty::chip::designer::Designer;
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State, // Used to access shared state
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use serde_json::json;
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex}, // For shared state
};
use tokio::net::TcpListener;

// 1. Define the application state
struct AppState {
    counter: u64,
    designer: Designer
}

// Type alias for the shared state
type SharedState = Arc<Mutex<AppState>>;

async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<SharedState>, // 2. Access shared state
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state)) // Pass state to the socket handler
}

async fn handle_socket(mut socket: WebSocket, state: SharedState) {
    println!("Client connected. Initializing session.");
    // Loop to process messages from the client
    while let Some(Ok(msg)) = socket.recv().await {
        match msg {
            Message::Text(text) => {
                if text == "tick" {
                    // 3. Update state and prepare response
                    let designer_state;
                    {
                        // Lock the state for safe mutation
                        let mut app_state = state.lock().unwrap();                        
                        let _ = app_state.designer.tick();
                        designer_state = app_state.designer.get_state();
                        // MutexGuard is dropped here, releasing the lock
                    }

                    // Create a JSON response including the updated state
                    let response = json!({
                        "state": "tock",
                        "designer": designer_state,
                    });

                    // Send the response back to the client
                    if socket
                        .send(Message::Text(response.to_string().into()))
                        .await
                        .is_err()
                    {
                        // Client disconnected or error sending
                        println!("Client disconnected or error sending. Ending session.");
                        return;
                    }
                } else {
                    // Handle other text messages if necessary
                    println!("Received unexpected text message: {}", text);
                    // Optionally send an error or ignore
                }
            }
            Message::Binary(bin) => {
                // Handle binary messages if necessary
                println!("Received binary message: {} bytes", bin.len());
            }
            Message::Ping(ping_data) => {
                // Axum handles pings by sending pongs automatically by default.
                // You can add custom logic here if needed.
                println!("Received ping");
                if socket.send(Message::Pong(ping_data)).await.is_err() {
                    println!("Failed to send pong. Client might be disconnected.");
                    return;
                }
            }
            Message::Pong(_) => {
                // Handle pong messages if necessary
                println!("Received pong");
            }
            Message::Close(close_frame) => {
                // Correctly handle Close messages
                if let Some(cf) = close_frame {
                    println!(
                        "Client sent close message: code={}, reason='{}'",
                        cf.code, cf.reason
                    );
                } else {
                    println!("Client sent close message without a frame.");
                }
                return; // Exit the loop and close the connection
            }
        }
    }
    println!("Client session ended (socket.recv() returned None or an error not caught above).");
}

#[tokio::main]
async fn main() {
    // Initialize the shared state
    let shared_state = Arc::new(Mutex::new(AppState { counter: 0, designer: Designer::new() }));

    // Build the Axum application router
    let app = Router::new()
        .route("/ws", get(websocket_handler))
        .with_state(shared_state); // Provide the state to the handlers

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("WebSocket server with state listening on ws://{}", addr);

    // Bind the TCP listener
    let listener = match TcpListener::bind(&addr).await {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Failed to bind to address {}: {}", addr, e);
            return;
        }
    };

    // Serve the application
    if let Err(e) = axum::serve(listener, app.into_make_service()).await {
        eprintln!("Server error: {}", e);
    }
}
