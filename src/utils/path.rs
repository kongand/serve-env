use clap::{Error, ErrorKind};

pub fn path(option: Option<&str>) -> String {
    let mut path = "env".to_owned();

    if let Some(path_option) = option {
        path = path_option.parse().unwrap();

        if path.contains("/") {
            let error = Error::with_description("Route argument (-r, --route) cannot contain slashes (/)\n", ErrorKind::InvalidValue);
            Error::exit(&error.unwrap());
        }
    }

    return path;
}
