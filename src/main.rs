use bytes::Bytes;
use http_body_util::Empty;
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::pin;

async fn redirect(
    redirect_url: Arc<String>,
    req: Request<Incoming>,
) -> Result<Response<Empty<Bytes>>, hyper::Error> {
    let path_and_query = req.uri().path_and_query().unwrap();
    println!("Redirecting to {}{}", &redirect_url, path_and_query);
    let full_url = format!("{}{}", &redirect_url, path_and_query);
    Ok(Response::builder()
        .status(301)
        .header("location", full_url)
        .body(Empty::new())
        .unwrap())
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

    // Use a 5 second timeout for incoming connections to the server.
    // If a request is in progress when the 5 second timeout elapses,
    // use a 2 second timeout for processing the final request and graceful shutdown.
    let connection_timeouts = vec![Duration::from_secs(5), Duration::from_secs(2)];

    loop {
        let redirect_url_task = redirect_url.clone();
        let (tcp, remote_address) = listener
            .accept()
            .await
            .expect("Failed to accept connection");
        let io = TokioIo::new(tcp);

        println!("Accepted connection from: {:?}", remote_address);

        let connection_timeouts_clone = connection_timeouts.clone();

        tokio::task::spawn(async move {
            let conn = http1::Builder::new().serve_connection(
                io,
                service_fn(move |req| redirect(redirect_url_task.clone(), req)),
            );
            pin!(conn);

            // Iterate the timeouts.  Use tokio::select! to wait on the
            // result of polling the connection itself,
            // and also on tokio::time::sleep for the current timeout duration.
            for sleep_duration in connection_timeouts_clone.iter() {
                tokio::select! {
                    res = conn.as_mut() => {
                        // Polling the connection returned a result.
                        // In this case print either the successful or error result for the connection
                        // and break out of the loop.
                        if let Err(e) = res {
                            println!("error serving connection: {:?}", e);
                        }
                        break;
                    }
                    _ = tokio::time::sleep(*sleep_duration) => {
                        // tokio::time::sleep returned a result.
                        // Call graceful_shutdown on the connection and continue the loop.
                        conn.as_mut().graceful_shutdown();
                    }
                }
            }
        });
    }
}
