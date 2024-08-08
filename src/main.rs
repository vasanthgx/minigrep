use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);
    // let query = &args[1];
    // let file_path = &args[2];

    // let (query, file_path) = parse_config(&args);

    let config = parse_config(&args);


    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path).expect("Failed to read file");

    println!( "With text:\n{}", contents);
}

 struct Config {
    query: String,
    file_path: String,
 }


fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config { query, file_path }
}