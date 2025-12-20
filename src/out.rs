use crate::http::Response;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

/// A text segment with associated styling
pub struct StyledSegment {
    text: String,
    color: Option<Color>,
    bold: bool,
    italic: bool,
}

impl StyledSegment {
    /// Create a new styled segment with just text (no styling)
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            color: None,
            bold: false,
            italic: false,
        }
    }

    /// Set the color
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Make the text bold
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    /// Make the text italic
    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    /// Add one space
    pub fn space(mut self) -> Self {
        self.text.push(' ');
        self
    }

    /// Add a newline
    pub fn newline(mut self) -> Self {
        self.text.push('\n');
        self
    }
}

/// A line composed of multiple styled segments
pub struct StyledLine {
    segments: Vec<StyledSegment>,
}

impl StyledLine {
    /// Create a new empty styled line
    pub fn new() -> Self {
        Self {
            segments: Vec::new(),
        }
    }

    /// Add a segment to the line
    pub fn add(mut self, segment: StyledSegment) -> Self {
        self.segments.push(segment);
        self
    }

    /// Print the line to stdout
    pub fn print(&self) {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);

        for segment in &self.segments {
            let mut color_spec = ColorSpec::new();

            if let Some(color) = segment.color {
                color_spec.set_fg(Some(color));
            }

            if segment.bold {
                color_spec.set_bold(true);
            }

            if segment.italic {
                color_spec.set_italic(true);
            }

            stdout.set_color(&color_spec).expect("Failed to set color");

            write!(&mut stdout, "{}", segment.text).expect("Failed to set color");
        }

        stdout // reest color and go to next line
            .set_color(ColorSpec::new().set_reset(true))
            .expect("Failed to reset color");
        writeln!(&mut stdout).expect("Failed to write to stdout");
    }
}

/// Display verbose information
pub fn display_request(method: &str, url: &str) {
    StyledLine::new()
        .add(StyledSegment::new(method).color(Color::Cyan).bold().space())
        .add(StyledSegment::new(url).color(Color::White))
        .print();
}

/// Display HTTP status code amd message
pub fn display_status(status: u16, status_message: &str) {
    StyledLine::new()
        .add(StyledSegment::new("Status").color(Color::Green).space())
        .add(
            StyledSegment::new(status.to_string())
                .color(Color::Blue)
                .bold()
                .space(),
        )
        .add(StyledSegment::new(status_message).color(Color::Green))
        .print();
}

/// Display Content-Type header
pub fn display_content_type(response: &Response) {
    if let Some(content_type) = response.headers.get("Content-Type") {
        StyledLine::new()
            .add(
                StyledSegment::new("Content-Type")
                    .color(Color::Green)
                    .space(),
            )
            .add(StyledSegment::new(content_type.to_str().unwrap()))
            .print();
    }
}

/// Display the complete response
pub fn display_response(
    response: &Response,
    formatted_body: &str,
    verbose: bool,
    method: &str,
    url: &str,
) {
    if verbose {
        display_request(method, url);
    }

    display_status(response.status, response.status_message);
    display_content_type(response);
    println!("{}", formatted_body);
}
