use std::error::Error;

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
