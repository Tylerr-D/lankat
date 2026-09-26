use std::sync::mpsc::Sender;
use std::sync::{ Arc, Mutex };
use serde::{ Serialize, Deserialize };
use axum::{
    extract::{
        State,
        ws::{ Message, WebSocket, WebSocketUpgrade },
    },
    http::header,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use tokio::{
    runtime::Runtime,
    sync::{
        mpsc,
    },
    net::{
        TcpListener
    },
};

use crate::network::net;

type Clients = Arc<Mutex<Vec<mpsc::Sender<String>>>>;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum WebPayload {
    Text  { sender: String, text: String },
    Image { sender: String, data: String },
}

#[derive(Clone)]
struct WebState {
    tx: Sender<net::Event>,
    clients: Clients,
}


const INDEX: &str = include_str!("index.html");
const STYLE: &str = include_str!("style.css");
const SCRIPT: &str = include_str!("app.js");


pub fn start(tx: Sender<net::Event>, mut outbox: mpsc::Receiver<String>) {
    std::thread::spawn(move || {
        let rt = Runtime::new().expect("tokio runtime");
        let clients: Clients = Arc::new(Mutex::new(Vec::new()));
        let state = WebState { tx, clients: clients.clone() };

        let drain = async move {
            while let Some(text) = outbox.recv().await {
                let mut list = clients.lock().unwrap();
                let mut alive = Vec::with_capacity(list.len());
                for client in list.drain(..) {
                    if client.send(text.clone()).await.is_ok() {
                        alive.push(client);
                    }
                }
                *list = alive;
            }
        };

        rt.block_on(async move {
            tokio::join!(serve(state.clone()), drain);
        });
    });
}

async fn serve(state: WebState) {
    let app = Router::new()
        .route("/", get(index))
        .route("/style.css", get(style))
        .route("/app.js", get(script))
        .route("/ws", get(ws_upgrade))
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("bind 8080");
    axum::serve(listener, app).await.expect("axum serve");
}


async fn index() -> Html<&'static str> {
    Html(INDEX)
}

async fn style() -> Response {
    ([(header::CONTENT_TYPE, "text/css")], STYLE).into_response()
}

async fn script() -> Response {
    ([(header::CONTENT_TYPE, "text/javascript")], SCRIPT).into_response()
}


async fn ws_upgrade(State(state): State<WebState>, ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(move |socket| {
        let (client_tx, client_rx) = mpsc::channel(64);
        state.clients.lock().unwrap().push(client_tx);
        ws_conn(socket, client_rx, state.tx.clone())
    })
}

async fn ws_conn(
    mut socket: WebSocket,
    mut client_rx: mpsc::Receiver<String>,
    tx:Sender<net::Event>
) {
    loop {
        tokio::select! {
            msg = socket.recv() => match msg {
                Some(Ok(Message::Text(raw_text))) => {
                    let Ok(payload) = serde_json::from_str::<WebPayload>(&raw_text) else {
                        tx.send(net::Event::WebMessage { text: raw_text.parse().unwrap() }).ok();
                        continue
                    };

                    match payload {
                        WebPayload::Text {  text, .. } => {
                            tx.send(net::Event::WebMessage { text }).ok();
                        },
                        WebPayload::Image { data, ..} => {
                            tx.send(net::Event::WebMessage {
                                text: format!("[Image sent from Web: {}...]", &data[..data.len().min(30)]),
                            }).ok();
                        }
                    }
                }
                Some(Ok(_)) => {}
                _ => break,
            },
            text = client_rx.recv() => match text {
                Some(json_payload) => {
                    if socket.send(Message::Text(json_payload.into())).await.is_err(){
                        break;
                    }
                }
                None => break,
            },
        }
    }
}
