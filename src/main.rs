use warp::Filter;
use std::env;
use std::collections::HashMap;
use clap::App;

#[tokio::main]
async fn main() {
    let matches = App::new("serve-env")
        .version("0.1.0")
        .author("Anders Gade <andersgade1994@gmail.com>")
        .about("This tool is a small web server that serves the machine's environment variables to the web.")
        .arg("-k, --key-prefix=[PREFIX] 'Set custom key prefix to serve. Default is REACT_APP_'")
        .get_matches();
    
    let mut prefix = "REACT_APP_";
    let prefix_option = matches.value_of("key-prefix");
    
    if prefix_option.is_some() {
        prefix = prefix_option.unwrap();
        println!("Serving env variables prefixed with {}", prefix);
    } else {
        println!("Serving env variables with default prefix ({})", prefix);
    }
    
    let mut response = HashMap::new();

    for (key, value) in env::vars() {
        if key.starts_with(prefix) {
            response.insert(key, value);
        }
    }

    let env = warp::path("env")
        .map(move || warp::reply::json(&response));
    
    let address = [127, 0, 0, 1];
    let port = 3030;

    println!("Running server at {:?}:{}", address, port);

    warp::serve(env)
        .run((address, port))
        .await;
}