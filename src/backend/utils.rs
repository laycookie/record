use std::collections::HashMap;

use serde::de::DeserializeOwned;
use surf::{Error, StatusCode};


pub enum Request_Type {
    GET,
    POST,
    PUT,
    DELETE,
}
pub async fn http_request<T: DeserializeOwned>(
    link: &str,
    headers: Vec<(&str, String)>,
    request_type: Request_Type,
) -> Result<T, surf::Error> {
    let mut req = match request_type {
        Request_Type::GET => surf::get(link),
        Request_Type::POST => surf::post(link),
        Request_Type::PUT => surf::put(link),
        Request_Type::DELETE => surf::delete(link),
    };

    for (key, value) in headers {
        req = req.header(key, value);
    }

    let mut res = req.send().await?;

    if StatusCode::Ok != res.status() {
        return Err(surf::Error::from_str(
            StatusCode::Unauthorized,
            "TODO: prob. an outdated token",
        ));
    }

    Ok(res.body_json::<T>().await?)
}
