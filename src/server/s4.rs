#![deny(warnings)]

use warp::{Filter};
use serde_derive::{Deserialize, Serialize};
use futures::{FutureExt, StreamExt};

#[derive(Deserialize, Serialize)]
struct Employee {
    name: String,
    rate: u32,
}

// 启动服务
// https://blog.csdn.net/knhony/article/details/109180885
pub async fn run_wrap() {
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}",name));

    // pretty_env_logger::init();

    let readme = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file("./README.md"));

    // dir already requires GET...
    let examples = warp::path("ex")
        .and(warp::fs::dir("./"));

    // POST /employees/:rate  {"name":"Sean","rate":2}
    let promote = warp::post()
        .and(warp::path("employees"))
        .and(warp::path::param::<u32>())
        // Only accept bodies smaller than 16kb...
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .map(|rate, mut employee: Employee| {
            employee.rate = rate;
            warp::reply::json(&employee)
        });

    let wss = warp::path("echo")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            ws.on_upgrade(|websocket| {
                // Just echo all messages back...
                let (tx,rx) = websocket.split();
                rx.forward(tx).map(|result| {
                    if let Err(e) = result {
                        eprintln!("websocket error: {:?}",e);
                    }
                })
            })
        });
        
    // GET / => README.md
    // GET /ex/... => ./
    // GET /hello/abc => hello()    

    let routes = readme
        .or(examples)
        .or(hello)
        .or(promote)
        .or(wss);
    warp::serve(routes)
        // .tls()
        // .cert_path("examples/tls/cert.pem")
        // .key_path("examples/tls/key.rsa")
        .run(([127,0,0,1],3030))
        .await;
}

// #[tokio::test]
// async fn redirect_uri() {
//     use warp::http::Uri;
//     let redirected = warp::any()
//     .map(|| warp::redirect(Uri::from_static("/over-there")));
//     let req = warp::test::request();
//     let resp = req.reply(&redirected).await;

//     assert_eq!(resp.status(),301);
//     assert_eq!(resp.headers()["location"], "/over-there")
// }