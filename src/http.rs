use reqwest;

/// HTTP response data
pub struct Response {
    pub status: u16,
    pub status_message: &'static str,
    pub headers: reqwest::header::HeaderMap,
    pub body: String,
}

/// Normalize url by adding https://
pub fn normalize_url(url: &str) -> String {
    if !url.starts_with("https://") && !url.starts_with("http://") {
        format!("https://{}", url)
    } else {
        url.to_string()
    }
}

/// Fetch with specified url, method and bodyy
pub async fn fetch(
    url: &str,
    method: &str,
    body: Option<String>,
) -> Result<Response, reqwest::Error> {
    let client = reqwest::Client::new();

    let mut request = match method {
        "GET" => client.get(url),
        "POST" => client.post(url),
        "PUT" => client.put(url),
        "DELETE" => client.delete(url),
        "PATCH" => client.patch(url),
        _ => client.get(url),
    };

    if let Some(body_content) = body {
        request = request
            .header("Content-Type", "application/json")
            .body(body_content);
    }

    let response = request.send().await?;

    let status = response.status().as_u16();
    let status_message = response
        .status()
        .canonical_reason()
        .expect("No status message");
    let headers = response.headers().clone();
    let body = response.text().await?;

    Ok(Response {
        status,
        status_message,
        headers,
        body,
    })
}
