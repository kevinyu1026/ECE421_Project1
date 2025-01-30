use warp::Filter;
use tokio::sync::{mpsc, Mutex};
use warp::ws::{Message, WebSocket};
use futures_util::{StreamExt, SinkExt};
use sqlx::{SqlitePool, query};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct Player {
    id: String,
    name: String,
}

struct ServerState {
    players: Mutex<HashMap<String, mpsc::UnboundedSender<Message>>>,
}

impl ServerState {
    fn new() -> Self {
        ServerState {
            players: Mutex::new(HashMap::new()),
        }
    }
}

#[tokio::main]
async fn main() {
    let db_pool = SqlitePool::connect("sqlite://poker.db")
        .await
        .expect("Failed to connect to database");

    let state = Arc::new(ServerState::new());

    let register_route = warp::path("ws")
        .and(warp::ws())
        .and(with_state(state.clone()))
        .and(with_db(db_pool.clone()))
        .map(|ws: warp::ws::Ws, state, db| ws.on_upgrade(move |socket| handle_connection(socket, state, db)));

    warp::serve(register_route).run(([10, 0, 0, 195], 3030)).await;
}

async fn handle_connection(ws: WebSocket, state: Arc<ServerState>, db: SqlitePool) {
    let (mut ws_tx, mut ws_rx) = ws.split();
    let (tx, mut rx) = mpsc::unbounded_channel();

    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            let _ = ws_tx.send(msg).await;
        }
    });

    while let Some(Ok(msg)) = ws_rx.next().await {
        if let Ok(text) = msg.to_str() {
            let player_name = text.trim().to_string();
            let player_id = Uuid::new_v4().to_string();

            // Insert into the database
            sqlx::query("INSERT INTO players (id, name) VALUES (?1, ?2)")
                .bind(&player_id)
                .bind(&player_name)
                .execute(&db)
                .await
                .expect("DB insert failed");

            // Store player connection in the server state
            let mut players = state.players.lock().await;
            players.insert(player_name.clone(), tx.clone());

            // Send a welcome message to the player
            let welcome_msg = Message::text(format!("Welcome, {}! You are registered.", player_name));
            tx.send(welcome_msg).unwrap();
        }
    }
}

fn with_state(state: Arc<ServerState>) -> impl Filter<Extract = (Arc<ServerState>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || state.clone())
}

fn with_db(db: SqlitePool) -> impl Filter<Extract = (SqlitePool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}
