use warp::Filter;
use std::str;
use std::env;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let mut response = HashMap::new();

    for (key, value) in env::vars() {
        response.insert(key, value);
    }

    let env = warp::path("env")
        .map(move || warp::reply::json(&response));

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    let routes = warp::get().and(hello.or(env));
    
    let address = [127, 0, 0, 1];
    let port = 3030;

    println!("Running server at {:?}:{}", address, port);

    warp::serve(routes)
        .run((address, port))
        .await;
}