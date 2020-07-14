mod utils;

use warp::Filter;
use clap::App;
use tokio::{signal, sync::oneshot};
use utils::{response, path};

#[tokio::main]
async fn main() {
    let matches = App::new("serve-env")
        .version("0.1.0")
        .author("Anders Gade <andersgade1994@gmail.com>")
        .about("This tool is a small web server that serves the machine's environment variables to the web.")
        .arg("-k, --key-prefix=[PREFIX] 'Set custom key prefix to serve. Default is REACT_APP_'")
        .arg("-p, --port=[PORT] 'Set custom port to serve on. Default is 3030'")
        .arg("-r, --route=[ROUTE] 'Set custom route to serve on. Default is /env'")
        .get_matches();

    let path = path(matches.value_of("route"));
    
    let address = [127, 0, 0, 1];
    let mut port = 3030;

    if let Some(port_option) = matches.value_of("port") {
        port = port_option.parse().unwrap();
    }

    let res = response("REACT_APP_", matches.value_of("key-prefix"));

    let env = warp::path(path.clone())
        .map(move || warp::reply::json(&res));

    println!("Running server at {:?}:{}/{}", address, port, path);

    let (tx, rx) = oneshot::channel();

    let (_addr, server) = warp::serve(env)
        .bind_with_graceful_shutdown((address, port), async {
            rx.await.ok();
        });

    tokio::task::spawn(server);

    let _ = signal::ctrl_c().await;
    println!("Ctrl + c received, closing server...");
    let __ = tx.send(());
}