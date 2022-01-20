use std::env;
use std::process;
use minigrep::Config;

fn main() { 
    let config: Config = Config::new(env::args()).unwrap_or_else(|err|{
        eprintln!("Проблема при разборе аргументов: {}", err);
        process::exit(1);
    });

    println!("Filename is --- {}", config.filename);
    println!("Searched query is --- {}", config.query);

    if let Err(e) = minigrep::run(config){
        eprintln!("error in app: {}", e);
        process::exit(1)
    }

}