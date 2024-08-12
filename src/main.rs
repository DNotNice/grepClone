use std::{env, process} ;
use grepClone:: Config ;
use grepClone:: run ;
fn main() {
    let args : Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("problem while parsing the arguments , {}" , err);
        process::exit(1);
        
    } );

    if let Err(e) = run(config) {
        print!("Application err : {}" , e);
        process::exit(1);
    }
}
