use std::env;
use cursed_text::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    match Config::new(&args){
        Ok(config) => println!("{}", config.curse()),
        Err(e) => println!("{}", e)
    }
    
}
