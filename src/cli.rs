use clap::{Parser, Subcommand};
use std::io::{self, Read};

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

        /// Request body data (JSON string, key=value pairs, or @file)
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        body_data: Vec<String>,

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

        /// Request body data (JSON string, key=value pairs, or @file)
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        body_data: Vec<String>,

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

        /// Request body data (JSON string, key=value pairs, or @file)
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        body_data: Vec<String>,

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
    pub fn get_body(&self) -> Result<Option<String>, String> {
        if let Some(ref cmd) = self.command {
            match cmd {
                Commands::Post {
                    body, body_data, ..
                } => Self::parse_body(body.clone(), body_data),
                Commands::Put {
                    body, body_data, ..
                } => Self::parse_body(body.clone(), body_data),
                Commands::Patch {
                    body, body_data, ..
                } => Self::parse_body(body.clone(), body_data),
                _ => Ok(None),
            }
        } else {
            Ok(self.body.clone())
        }
    }

    /// Parse body from either -b flag or positional args
    fn parse_body(
        flag_body: Option<String>,
        body_data: &[String],
    ) -> Result<Option<String>, String> {
        if let Some(body) = flag_body {
            return Ok(Some(body));
        }

        if body_data.is_empty() {
            if !atty::is(atty::Stream::Stdin) {
                let mut buffer = String::new();
                io::stdin()
                    .read_to_string(&mut buffer)
                    .map_err(|e| format!("Failed to read from stdin: {}", e))?;
                if !buffer.is_empty() {
                    return Ok(Some(buffer));
                }
            }
            return Ok(None);
        }

        if body_data.len() == 1 {
            let arg = &body_data[0];

            if let Some(file_path) = arg.strip_prefix('@') {
                return std::fs::read_to_string(file_path)
                    .map(Some)
                    .map_err(|e| format!("Failed to read file '{}': {}", file_path, e));
            }

            return Ok(Some(arg.clone()));
        }

        let mut json_obj = serde_json::Map::new();
        for arg in body_data {
            if let Some((key, value)) = arg.split_once('=') {
                let json_value = match serde_json::from_str(value) {
                    Ok(v) => v,
                    Err(_) => serde_json::Value::String(value.to_string()),
                };
                json_obj.insert(key.to_string(), json_value);
            } else {
                return Err(format!("Invalid key=value pair: '{}'", arg));
            }
        }

        Ok(Some(serde_json::to_string(&json_obj).unwrap()))
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
