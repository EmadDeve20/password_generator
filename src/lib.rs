use clap::Parser;
use rand::Rng;
use std::{collections::BTreeSet, iter::FromIterator};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Length of Password
    #[arg(short, long, default_value_t = 8)]
    pub length: u8,

    /// Filter Characters
    #[arg(short, long, default_value_t = String::from(""))]
    pub filter: String,
}

fn string_to_char_list(string: String) -> Vec<char> {
    
    string.chars().collect()
} 

fn remove_vector_in_vector(mut items: Vec<char>, to_remove: Vec<char>) -> Vec<char> {

    let to_remove = BTreeSet::from_iter(to_remove);

    items.retain(|e| !to_remove.contains(e));

    items
}

fn get_characters(char_filters: Vec<char>) -> Vec<char> {

    let characters = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g',
    'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't','u', 'v', 'w',
    'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
    'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Z', 'Y', 'Z',
    '~', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '_',
    '=', '+', '`', '\'', '"','|', ';', ':', '>', '<', '.', '/', '?', '\\'];

    if char_filters.len() == 0 {return characters}
    else {
        remove_vector_in_vector(characters, char_filters)
    }

}

pub fn generate_password(len: u8, string: String) -> String {
    
    let characters = get_characters(string_to_char_list(string));
    let mut password = String::new();


    for _ in 0..len {
        let random = rand::thread_rng().gen_range(0..characters.len());

        password = format!("{}{}", password, characters[random]);
    }

    password
}