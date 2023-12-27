use std::fs;
pub fn set_up_server<'a>() {
    let resources = find_resources();
    resources.into_iter().for_each(|r| println!("{}", r));
}

// TODO: actually do something with this
fn find_resources() -> Vec<String> {
    let mut resources: Vec<String> = vec![];
    let paths = fs::read_dir("./resources").unwrap();
    for path in paths {
        let path_temp = path.unwrap().path();
        let test = path_temp.to_str().unwrap();
        println!("{}", test);
        resources.push(test.to_string());
    }
    return resources;
}
