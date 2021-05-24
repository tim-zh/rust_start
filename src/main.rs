#![deny(warnings)]

use warp::Filter;
//use futures::{FutureExt, StreamExt};

#[tokio::main]
async fn main() {
    println!("{}", std::env::current_dir().unwrap().display());
    /*let routes = warp::path::end().and(warp::fs::dir("."))
        .or(
            warp::path("echo")
                .and(warp::ws())
                .map(|ws: warp::ws::Ws| {
                    ws.on_upgrade(|websocket| {
                        let (tx, rx) = websocket.split();
                        rx.forward(tx).map(|result| {
                            if let Err(e) = result {
                                eprintln!("websocket error: {:?}", e);
                            }
                        })
                    })
                })
        );*/

    webbrowser::open("http://localhost:8000/index.html").unwrap();

    //warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;

    let readme = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file("./README.md"));
    let examples = warp::path("ex").and(warp::fs::dir("./examples/"));
    let routes = readme.or(examples);
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}