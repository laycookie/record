use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use serde::de::DeserializeOwned;
use surf::{RequestBuilder, StatusCode};

pub async fn http_request<T: DeserializeOwned>(
    mut req: RequestBuilder,
    headers: Vec<(&str, String)>,
) -> Result<T, Box<dyn Error>> {
    for (key, value) in headers {
        req = req.header(key, value);
    }

    let mut res = req.send().await?;

    if StatusCode::Ok != res.status() {
        return Err(surf::Error::from_str(
            StatusCode::Unauthorized,
            "TODO: prob. an outdated token",
        )
        .into());
    }

    let json = res.body_json::<T>().await?;
    Ok(json)
}
pub async fn cache_download(url: String, path: PathBuf, file_name: String) {
    // Send HTTP GET request to the specified URL
    let req = surf::get(&url);
    let mut res = req.send().await.unwrap();

    let StatusCode::Ok = res.status() else {
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