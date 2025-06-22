mod datamodel;

use futures_util::stream::SplitSink;
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::ws::{Message, WebSocket};
use warp::Filter;
use webrtc::peer_connection::sdp::session_description::RTCSessionDescription;

type PeerId = String;
type Cache = Arc<
    Mutex<
        HashMap<
            PeerId,
            (
                Arc<Mutex<RTCSessionDescription>>,
                Arc<Mutex<SplitSink<WebSocket, Message>>>,
            ),
        >,
    >,
>;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PeerRequest {
    peer_id: PeerId,
    session_description: RTCSessionDescription,
}

async fn ws_handler(ws: WebSocket, cache: Cache) {
    let (ws_tx, mut ws_rx) = ws.split();
    let tx = Arc::new(Mutex::new(ws_tx));

    // Handle incoming messages from the WebSocket
    while let Some(result) = ws_rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("websocket error for peer: {}", e);
                break;
            }
        };

        if let Ok(text) = msg.to_str() {
            println!("Received message: {}", text);

            let input: serde_json::error::Result<RTCSessionDescription> =
                serde_json::from_str(text);
            if let Ok(offer) = input {
                let peer_id = generate_unique_id(&cache).await;
                let arc = cache.clone();
                let mut guard = arc.lock().await;
                guard.insert(peer_id.clone(), (Arc::new(Mutex::new(offer)), tx.clone()));

                tx.clone()
                    .lock()
                    .await
                    .send(Message::text(peer_id.clone()))
                    .await
                    .expect("Failed to send generated ID to the seed");

                println!("Registered new SEED: {}", peer_id.as_str());
            } else if let Ok(req) = serde_json::from_str::<PeerRequest>(text) {
                let peer_id = req.peer_id.as_str();
                let arc = cache.clone();
                let guard = arc.lock().await;

                let (offer, seed_channel) = guard
                    .get(peer_id)
                    .expect(format!("Failed to find cached SEED for id {peer_id}").as_str());
                let offer = serde_json::to_string(offer.clone().lock().await.deref())
                    .expect("Could not serialize session description");
                let answer = serde_json::to_string(&req.session_description)
                    .expect("Could not serialize session description");
                
                let clone_seed_channel = seed_channel.clone();
                tokio::spawn(async move {
                    clone_seed_channel
                        .lock()
                        .await
                        .send(Message::text(answer))
                        .await
                        .expect("Failed to send the answer to the seed");
                });
                let cloned_peer_channel = tx.clone();
                tokio::spawn(async move {
                    cloned_peer_channel
                        .lock()
                        .await
                        .send(Message::text(offer))
                        .await
                        .expect("Failed to send the offer to the peer");
                });

                println!("Facilitated a new P2P connection for SEED[{peer_id}]");
            } else {
                println!("Received bad request from: {}", text);
            }
        }
    }
}

async fn generate_unique_id(cache: &Cache) -> String {
    let cache = cache.lock().await;
    loop {
        let id = uuid::Uuid::new_v4().to_string();
        if !cache.contains_key(&id) {
            break id;
        }
    }
}
// --- Main Server Setup ---

#[tokio::main]
async fn main() {
    let cache: Cache = Arc::new(Mutex::new(HashMap::new()));

    // Clone the `rooms` Arc for each incoming connection.
    let rooms_filter = warp::any().map(move || Arc::clone(&cache));

    // Define the WebSocket route: GET /ws
    let websocket_route =
        warp::path("ws")
            .and(warp::ws())
            .and(rooms_filter)
            .map(|ws: warp::ws::Ws, cache| {
                ws.on_upgrade(move |websocket| ws_handler(websocket, cache))
            });

    println!("Signaling server running on ws://127.0.0.1:3030/ws");
    warp::serve(websocket_route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
