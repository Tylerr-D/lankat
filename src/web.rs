use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::{Html, Response},
    routing::get,
    Router,
};

pub fn start(){
    std::thread::spawn(|| {
        let rt = tokio::runtime::Runtime::new().expect("tokio runtime");
        rt.block_on(serve());
    });
}

async fn serve(){
    let app = Router::new()
    .route("/", get(index))
    .route("/ws", get(ws_upgrade));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
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


async fn ws_conn(mut socket: WebSocket){
    while let SOme(Ok(Message::Text(text))) = socket.recv().await {
        let reply = format!("echo: {}", text.to_string());
        let _ = socket.send(Message::Text(reply.into())).await;
    }
}

const INDEX: &str = include_str!("index.html");
const STYLE: &str = include_str!("style.css");
const SCRIPT: &str = include_str!("app.js");