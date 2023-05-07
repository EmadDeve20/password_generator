use clap::Parser;
use password_generator::{Args, generate_password};

fn main() {

    let args = Args::parse();

    println!("Password: {}", generate_password(args.length, args.filter));
}
