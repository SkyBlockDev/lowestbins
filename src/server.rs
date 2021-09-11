use hyper::service::{make_service_fn, service_fn};
use hyper::{header, Body, Method, Request, Response, Result, Server};
use log::{error, info};
use substring::Substring;
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

static INDEX: &str = "lowestbins.json";
static NOTFOUND: &[u8] = b"Not Found";

pub async fn start_server() {
    let addr = "127.0.0.1:1337".parse().unwrap();

    let make_service =
        make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(response_examples)) });

    let server = Server::bind(&addr).serve(make_service);

    info!("Listening on http://{}", addr);
    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        error!("server error: {}", e);
    }
}

async fn response_examples(req: Request<Body>) -> Result<Response<Body>> {
    info!("{} {}", req.method(), req.uri().path().substring(0, 30));

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/lowestbins.json") | (&Method::GET, "/lowestbins") => {
            simple_file_send(INDEX).await
        }
        _ => Ok(not_found()),
    }
}

/// HTTP status code 404
fn not_found() -> Response<Body> {
    Response::builder()
        // .status(StatusCode::NOT_FOUND)
        .body(NOTFOUND.into())
        .unwrap()
}

async fn simple_file_send(filename: &str) -> Result<Response<Body>> {
    // Serve a file by asynchronously reading it by chunks using tokio-util crate.

    if let Ok(file) = File::open(filename).await {
        let stream = FramedRead::new(file, BytesCodec::new());
        let body = Body::wrap_stream(stream);

        return Ok(Response::builder()
            .header(header::CONTENT_TYPE, "application/json")
            .body(body)
            .unwrap());
    }

    Ok(not_found())
}
