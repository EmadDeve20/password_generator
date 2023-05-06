use clap::Parser;
use rand::Rng;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Length of Password
    #[arg(short, long, default_value_t = 8)]
    pub length: u8,
}

fn get_characters() -> Vec<char> {
    vec!['a', 'b', 'c', 'd', 'e', 'f', 'g',
    'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't','u', 'v', 'w',
    'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
    'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Z', 'Y', 'Z',
    '~', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '_',
    '=', '+', '`', '\'', '"','|', ';', ':', '>', '<', '.', '/', '?', '\\']
}

pub fn generate_password(len: u8) -> String {
    
    let characters = get_characters();
    let mut password = String::new();


    for _ in 0..len {
        let random = rand::thread_rng().gen_range(0..characters.len());

        password = format!("{}{}", password, characters[random]);
    }

    password
}