use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::env;
use std::net::SocketAddr;

async fn redirect(
    redirect_url: String,
    req: Request<Body>,
) -> Result<Response<Body>, hyper::Error> {
    let path_and_query = req.uri().path_and_query().unwrap();
    println!("Redirecting to {}{}", &redirect_url, path_and_query);
    let full_url = format!("{}{}", &redirect_url, path_and_query);
    Ok(Response::builder()
        .status(301)
        .header("location", full_url)
        .body(Body::empty())
        .unwrap())
}

async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_owned());
    let address = SocketAddr::from(([0, 0, 0, 0], port.parse::<u16>().expect("Invalid port")));
    let redirect_url =
        env::var("REDIRECT_URL").unwrap_or_else(|_| "https://example.net".to_owned());

    let make_service = make_service_fn(move |_conn| {
        let redirect_url = redirect_url.clone();
        async move { Ok::<_, hyper::Error>(service_fn(move |req| redirect(redirect_url.clone(), req))) }
    });

    let server = Server::bind(&address).serve(make_service);
    println!("Listening on http://{}", address);

    let graceful = server.with_graceful_shutdown(shutdown_signal());

    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    }
}
