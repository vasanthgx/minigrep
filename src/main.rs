use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);
    // let query = &args[1];
    // let file_path = &args[2];

    // let (query, file_path) = parse_config(&args);

    // let config = parse_config(&args);

    let config = Config::new(&args);


    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path).expect("Failed to read file");

    println!( "With text:\n{}", contents);
}


///////////////////////Grouping configuration values with a Struct///////////////////

 struct Config {
    query: String,
    file_path: String,
 }

 ////////////////////Extracting the Argument parser ///////////////////////////

// fn parse_config(args: &[String]) -> Config {
//     let query = args[1].clone();
//     let file_path = args[2].clone();

//     Config { query, file_path }
// }



///////////////////////Creating a construcgtor for Config/////////////////

impl Config {
    fn new(args: &[String]) -> Config{
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path}
    }
}

