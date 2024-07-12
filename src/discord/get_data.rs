use reqwest::header::HeaderValue;

pub fn get_friendlist_ids() {
    println!("Get");
}
pub async fn get_login_by_token(url: &str, header:HeaderValue) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header("Authorization", header)
        .send()
        .await?
        .text()
        .await?;
    println!("{}", response);
    Ok(response)
}