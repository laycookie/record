use std::collections::HashMap;

use serde::de::DeserializeOwned;
use surf::{Error, StatusCode};

pub async fn http_get<T: DeserializeOwned>(
    link: &str,
    headers: Vec<(&str, String)>,
) -> Result<T, surf::Error> {
    let mut req = surf::get(link);

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
