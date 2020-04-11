use bytes::Bytes;
use warp::{
    http::StatusCode,
    Filter,
};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    #[structopt(short, long, default_value = "8000")]
    port: u16,
}

#[tokio::main]
async fn main() {
    let args = Args::from_args();

    let port = args.port;

    let file = warp::fs::dir(".").and(warp::get());

    let script = warp::path!("save")
        .and(warp::post())
        .and(warp::body::bytes())
        .map(|bytes: Bytes| {
            match std::fs::write("sav.yml", &bytes) {
                Ok(_) => StatusCode::OK,
                Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
            }
        });

    let routes = file.or(script);

    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
}

