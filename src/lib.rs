use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Length of Password
    #[arg(short, long, default_value_t = 8)]
    pub length: u8,
}
