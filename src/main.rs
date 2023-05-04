use rand::Rng;
use std::env;

fn main() {

    let args:Vec<String> = env::args().collect();

    let len_password;

    let characters = ['a', 'b', 'c', 'd', 'e', 'f', 'g',
    'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w',
    'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
    'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Z', 'Y', 'Z',
    '~', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '_',
    '=', '+', '`', '\'', '"','|', ';', ':', '>', '<', '.', '/', '?', '\\'];

    if args.len() == 2 {
        len_password = args[1].parse().unwrap();
    } else {
        len_password = 8;
    }

    
    
    for _ in 0..len_password {
        let random = rand::thread_rng().gen_range(0..characters.len());

        print!("{}", characters[random]);
    }

    println!("\n");

}
