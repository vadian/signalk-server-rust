use std::convert::Infallible;
use std::net::SocketAddr;
use log::debug;
use hyper::{Method, StatusCode};
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};

async fn health_check(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("SignalK Server OK".into()))
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(health_check))
    });

    let server = Server::bind(&addr).serve(make_svc);
    println!("Server start: {}", server.local_addr());

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}