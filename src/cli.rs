use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(args_conflicts_with_subcommands = true)]
pub struct Args {
    /// URL to request
    #[arg(value_name = "URL")]
    pub(crate) url: Option<String>,

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

    /// HTTP method subcommand
    #[command(subcommand)]
    pub(crate) command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Send a GET request
    Get {
        /// URL to request
        url: String,

        /// Verbose mode
        #[arg(short, long, default_value_t = 0)]
        verbose: u8,
    },
    /// Send a POST request
    Post {
        /// URL to request
        url: String,

        /// Request body (JSON string or key=value pairs)
        #[arg(short = 'b', long = "body")]
        body: Option<String>,

        /// Verbose mode
        #[arg(short, long, default_value_t = 0)]
        verbose: u8,
    },
    /// Send a PUT request
    Put {
        /// URL to request
        url: String,

        /// Request body (JSON string or key=value pairs)
        #[arg(short = 'b', long = "body")]
        body: Option<String>,

        /// Verbose mode
        #[arg(short, long, default_value_t = 0)]
        verbose: u8,
    },
    /// Send a DELETE request
    Delete {
        /// URL to request
        url: String,

        /// Verbose mode
        #[arg(short, long, default_value_t = 0)]
        verbose: u8,
    },
    /// Send a PATCH request
    Patch {
        /// URL to request
        url: String,

        /// Request body (JSON string or key=value pairs)
        #[arg(short = 'b', long = "body")]
        body: Option<String>,

        /// Verbose mode
        #[arg(short, long, default_value_t = 0)]
        verbose: u8,
    },
}

impl Args {
    /// Get the HTTP method from the flags or subcommand
    pub fn method(&self) -> &str {
        if let Some(ref cmd) = self.command {
            match cmd {
                Commands::Get { .. } => "GET",
                Commands::Post { .. } => "POST",
                Commands::Put { .. } => "PUT",
                Commands::Delete { .. } => "DELETE",
                Commands::Patch { .. } => "PATCH",
            }
        } else if self.post {
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

    /// Get the URL from either subcommand or root argument
    pub fn get_url(&self) -> &str {
        if let Some(ref cmd) = self.command {
            match cmd {
                Commands::Get { url, .. } => url,
                Commands::Post { url, .. } => url,
                Commands::Put { url, .. } => url,
                Commands::Delete { url, .. } => url,
                Commands::Patch { url, .. } => url,
            }
        } else {
            self.url.as_ref().expect("URL is required")
        }
    }

    /// Get the body from either subcommand or root argument
    pub fn get_body(&self) -> Option<String> {
        if let Some(ref cmd) = self.command {
            match cmd {
                Commands::Post { body, .. } => body.clone(),
                Commands::Put { body, .. } => body.clone(),
                Commands::Patch { body, .. } => body.clone(),
                _ => None,
            }
        } else {
            self.body.clone()
        }
    }

    /// Get verbose level from either subcommand or root argument
    pub fn get_verbose(&self) -> u8 {
        if let Some(ref cmd) = self.command {
            match cmd {
                Commands::Get { verbose, .. } => *verbose,
                Commands::Post { verbose, .. } => *verbose,
                Commands::Put { verbose, .. } => *verbose,
                Commands::Delete { verbose, .. } => *verbose,
                Commands::Patch { verbose, .. } => *verbose,
            }
        } else {
            self.verbose
        }
    }
}
