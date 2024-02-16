use axum::extract::State;
use axum::http::StatusCode;
use axum::http::Uri;
use axum::response::Redirect;
use axum::Router;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::signal;
use tower::ServiceBuilder;
use tower_http::trace;
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer, ServiceBuilderExt};
use tracing::Level;

#[derive(Clone, Debug)]
pub struct HttpServerState {
    pub redirect_url: Arc<String>,
}

async fn redirect(State(state): State<HttpServerState>, uri: Uri) -> Result<Redirect, StatusCode> {
    let path_and_query = uri.path_and_query().unwrap().as_str();
    let redirect_url = state.redirect_url;
    //println!("Redirecting to {}{}", redirect_url, path_and_query);
    //let full_url = format!("{}{}", redirect_url, path_and_query);
    let mut full_url = String::with_capacity(path_and_query.len() + redirect_url.len());
    full_url.push_str(&redirect_url);
    full_url.push_str(path_and_query);
    Ok(Redirect::permanent(&full_url))
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_owned());
    let address = SocketAddr::from(([0, 0, 0, 0], port.parse::<u16>().expect("Invalid port")));
    let redirect_url =
        Arc::new(env::var("REDIRECT_URL").unwrap_or_else(|_| "https://example.net".to_owned()));

    let listener = TcpListener::bind(&address)
        .await
        .expect("Failed to bind to address");
    println!("Listening on http://{}", address);

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    // Middleware creation
    let middleware = ServiceBuilder::new()
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(TimeoutLayer::new(Duration::from_secs(10)))
        .compression()
        .into_inner();

    let state = HttpServerState { redirect_url };

    let app = Router::new()
        .fallback(redirect)
        .layer(middleware)
        .with_state(state);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("Failed to start server");
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
