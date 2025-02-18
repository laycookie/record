use serde::de::DeserializeOwned;
use surf::{RequestBuilder, StatusCode};

pub async fn http_request<T: DeserializeOwned>(
    mut req: RequestBuilder,
    headers: Vec<(&str, String)>,
) -> Result<T, surf::Error> {
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

    res.body_json::<T>().await
}
