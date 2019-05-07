// use std::env;
use std::process;
use shcwell;

fn main() {
    // let args: Vec<String> = env::args().collect();

    // let cfg = Config::new(&args).unwrap_or_else(|err| {
    //     eprintln!("Could not parse arguments: {}", err);
    //     process::exit(1);
    // });

    if let Err(e) = shcwell::run() {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
