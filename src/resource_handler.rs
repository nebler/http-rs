use std::{env, fs};
pub struct Resource<'a> {
    url: &'a str,
    path: &'a str,
}

pub fn setUpServer<'a>() -> Vec<Resource<'a>> {
    return vec![];
}

fn find_resources() -> Vec<String> {
    let mut resources: Vec<String> = vec![];
    let paths = fs::read_dir("./").unwrap();
    for path in paths {
        let path_name = match path {
            Ok(t) => t,
            Err(e) => unwrap_failed("called `Result::unwrap()` on an `Err` value", &e),
        }
        .path()
        .to_str()
        .unwrap();
        println!("{}", path_name);
        resources.push(path_name.to_string());
    }
    return resources;
}
