use clap::Parser;
use rand::Rng;
use password_generator::Args;

fn main() {

    let args = Args::parse();


    let len_password = args.length;

    let characters = ['a', 'b', 'c', 'd', 'e', 'f', 'g',
    'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w',
    'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
    'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Z', 'Y', 'Z',
    '~', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '_',
    '=', '+', '`', '\'', '"','|', ';', ':', '>', '<', '.', '/', '?', '\\'];

        
    for _ in 0..len_password {
        let random = rand::thread_rng().gen_range(0..characters.len());

        print!("{}", characters[random]);
    }

    println!("\n");

}
