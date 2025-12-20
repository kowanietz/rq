mod cli;
mod http;
mod out;
mod response;

use crate::cli::Args;
use clap::Parser;

#[tokio::main]
async fn main() {
    let args = Args::parse(); // Collects cli arugments and parses them

    let url = http::normalize_url(args.get_url());

    let method = args.method(); // Extracts method and body
    let body = args.get_body().unwrap_or_else(|e| {
        eprintln!("Error parsing body: {}", e);
        std::process::exit(1);
    });

    let response = http::fetch(&url, method, body)
        .await
        .expect("Failed to fetch URL");

    // Pretty-print body (if josn)
    let formatted_body = response::format_body(&response.body);

    let verbose = args.get_verbose() >= 1;
    out::display_response(&response, &formatted_body, verbose, method, &url);
}
