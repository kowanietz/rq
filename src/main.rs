mod cli;
mod out;

use crate::cli::Args;
use crate::out::{StyledLine, StyledSegment};
use clap::Parser;
use reqwest;
use termcolor::Color;

struct Response {
    status: u16,
    status_message: &'static str,
    headers: reqwest::header::HeaderMap,
    body: String,
}

async fn fetch_url(
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

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let url: String = if !args.url.starts_with("https://") {
        format!("https://{}", args.url)
    } else {
        args.url.clone()
    };

    let method = args.method();
    let body = args.body.clone();

    let res = fetch_url(&url, method, body)
        .await
        .expect("Failed to fetch URL");

    let pretty_body: String;
    match serde_json::from_str::<serde_json::Value>(&res.body) {
        Ok(json) => pretty_body = serde_json::to_string_pretty(&json).unwrap(),
        Err(_) => pretty_body = res.body,
    }

    if args.verbose == 1 {
        StyledLine::new()
            .add(StyledSegment::new(method).color(Color::Cyan).bold().space())
            .add(StyledSegment::new(&url).color(Color::White))
            .print();
    }

    StyledLine::new()
        .add(StyledSegment::new("Status").color(Color::Green).space())
        .add(
            StyledSegment::new(res.status.to_string())
                .color(Color::Blue)
                .bold()
                .space(),
        )
        .add(StyledSegment::new(res.status_message).color(Color::Green))
        .print();

    if let Some(content_type) = res.headers.get("Content-Type") {
        StyledLine::new()
            .add(
                StyledSegment::new("Content-Type")
                    .color(Color::Green)
                    .space(),
            )
            .add(StyledSegment::new(content_type.to_str().unwrap()))
            .print();
    }

    println!("{}", pretty_body);
}
