use std::fs;
use std::path::Path;

pub fn verify_paths(server_dir: &str, client_dir: &str, file_name: &str) -> (String, String) {
    if [server_dir, client_dir].iter().any(|dir| !Path::new(dir).exists()) {
        panic!("Couldn't find one or more of the directories to save the resulting file to");
    }

    (format!("{}/{}", server_dir, file_name), format!("{}/{}", client_dir, file_name))
}

pub fn write_string(paths: (String, String), content: String) {
    fs::write(paths.0, content.clone()).expect("Failed to save data for server");
    fs::write(paths.1, content).expect("Failed to save data for client");
}
