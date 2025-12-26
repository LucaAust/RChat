use axum::extract::ws::{Message, WebSocket};
use axum::{extract::ws::WebSocketUpgrade, response::Response};

pub async fn ws_upgrade_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    let _ = socket.send(Message::Text("Welcome!".into())).await;

    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg {
            match &msg {
                Message::Text(t) => {
                    println!("{t}");
                }
                _ => {}
            }

            Message::Text("Got ya!".into())
        } else {
            // client disconnected
            return;
        };

        if socket.send(msg).await.is_err() {
            // client disconnected
            return;
        }
    }
}
