use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
        http::header,
    response::{Html, IntoResponse, Response},
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
    .route("/style.css", get(style))
    .route("/app.js", get(script))
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

async fn ws_upgrade(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(ws_conn)
}

async fn ws_conn(mut socket: WebSocket){
    while let Some(Ok(_)) = socket.recv().await {}
}

const INDEX: &str = include_str!("index.html");
const STYLE: &str = include_str!("style.css");
const SCRIPT: &str = include_str!("app.js");