use std::env;
use std::process;

use minigrep::Config;
fn main() {
    let args: Vec<String> = env::args().collect();
    //dbg!(args);

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(404);
    });

    /*s
    let query = &args[1];s
    let file_path = &args[2];
    */
    println!("Searching for {}",config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) { //if let是用来匹配是否存在错误的.
        eprintln!("Application for error: {e}");
        process::exit(1);
    }

}

