use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);
    let query = &args[1];
    let file_pathe = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_pathe);
}
