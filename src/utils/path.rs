pub fn path(option: Option<&str>) -> String {
    let mut path = "env".to_owned();

    if let Some(path_option) = option {
        path = path_option.parse().unwrap();
    }

    return path;
}
