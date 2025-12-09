mod cli;
mod out;

use crate::out::{StyledLine, StyledSegment};
use clap::Parser;
use cli::Args;
use reqwest;
use termcolor::Color;

struct Response {
    status: u16,
    status_message: &'static str,
    headers: reqwest::header::HeaderMap,
    body: String,
}

async fn fetch_url(url: &str) -> Result<Response, reqwest::Error> {
    let response = reqwest::get(url).await?;

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

    let url: String;

    if !args.url.starts_with("https://") {
        url = format!("https://{}", args.url);
    } else {
        url = args.url;
    }

    let res = fetch_url(&url).await.expect("Failed to fetch URL");

    let pretty_body: String;
    match serde_json::from_str::<serde_json::Value>(&res.body) {
        Ok(json) => pretty_body = serde_json::to_string_pretty(&json).unwrap(),
        Err(_) => pretty_body = res.body,
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

    StyledLine::new()
        .add(
            StyledSegment::new("Content-Type")
                .color(Color::Green)
                .space(),
        )
        .add(StyledSegment::new(
            res.headers
                .get("Content-Type")
                .expect("Header not found")
                .to_str()
                .unwrap(),
        ))
        .print();

    println!("{}", pretty_body);
}
