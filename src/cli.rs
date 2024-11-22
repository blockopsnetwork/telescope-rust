use clap::{Parser};

#[derive(Parser)]
#[command(name = "Node Monitor")]
#[command(about = "A tool for monitoring node metrics", long_about = None)]
pub struct Cli {
    #[arg(short, long, default_value = "9100")]
    pub port: u16, // Port to serve the HTTP endpoint.

    #[arg(short, long)]
    pub display: bool, // If true, display metrics in the terminal.
}

pub fn parse_args() -> Cli {
    Cli::parse()
}
