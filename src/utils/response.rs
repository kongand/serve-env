use std::collections::HashMap;
use std::env;

pub fn response(default_prefix: &str, prefix_option: Option<&str>) -> HashMap<String, String> {
    let prefix;
    
    if prefix_option.is_some() {
        prefix = prefix_option.unwrap();
        println!("Serving env variables prefixed with {}", prefix);
    } else {
        prefix = default_prefix;
        println!("Serving env variables with default prefix ({})", prefix);
    }
    
    let mut response = HashMap::new();

    for (key, value) in env::vars() {
        if key.starts_with(prefix) {
            response.insert(key, value);
        }
    }

    return response;
}
