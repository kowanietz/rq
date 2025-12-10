use clap::Parser;

/// HTTP request CLI tool
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// URL to request
    pub(crate) url: String,

    /// GET request
    #[arg(short = 'g', long = "get", group = "method")]
    pub(crate) get: bool,

    /// POST request
    #[arg(short = 'p', long = "post", group = "method")]
    pub(crate) post: bool,

    /// PUT request
    #[arg(short = 'u', long = "put", group = "method")]
    pub(crate) put: bool,

    /// DELETE request
    #[arg(short = 'd', long = "delete", group = "method")]
    pub(crate) delete: bool,

    /// PATCH request
    #[arg(short = 'x', long = "patch", group = "method")]
    pub(crate) patch: bool,

    /// Request body (JSON string or key=value pairs)
    #[arg(short = 'b', long = "body")]
    pub(crate) body: Option<String>,

    /// Verbose mode
    #[arg(short, long, default_value_t = 0)]
    pub(crate) verbose: u8,
}

impl Args {
    /// Get the HTTP method from the flags
    pub fn method(&self) -> &str {
        if self.post {
            "POST"
        } else if self.put {
            "PUT"
        } else if self.delete {
            "DELETE"
        } else if self.patch {
            "PATCH"
        } else {
            "GET"
        }
    }
}
