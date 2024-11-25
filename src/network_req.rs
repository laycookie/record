use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use slint::SharedString;

pub async fn cache_download(url: &SharedString, path: PathBuf, file_name: &SharedString) {
    // Send HTTP GET request to the specified URL
    let req = surf::get(&url);
    let mut res = req.send().await.unwrap();

    let surf::StatusCode::Ok = res.status() else {
        panic!("{}", url);
    };

    // Create a file at the specified path
    println!("{:#?}", path);
    if !path.exists() {
        match fs::create_dir_all(path.clone()) {
            Ok(_) => println!("Directory created successfully."),
            Err(e) => println!("Failed to create directory: {}", e),
        }
    }

    let mut file = File::create(path.join(file_name.to_string())).unwrap();

    // Copy the content from the response to the file
    let bytes = res.body_bytes().await.unwrap();
    file.write_all(&bytes).unwrap();
}
