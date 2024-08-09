use std::{env, error::Error, fs, process} ;

fn main() {
    let args : Vec<String> = env::args().collect();

    let config = get_args(args).unwrap_or_else(|err|{
        println!("problem while parsing the arguments , {}" , err);
        process::exit(1);
        
    } );

    if let Err(e) = run(config) {
        print!("Application err : {}" , e);
        process::exit(1);
    }
}

fn run(config : Config) -> Result<() , Box< dyn Error>> {
   let content = fs::read_to_string(config.file_name)?;
   println!("reading query {} in file\n {}" ,config.query ,  content);
   Ok(())
}

fn get_args(args : Vec<String>) -> Result<Config , String>{
    if args.len() < 3 {
      return Err("not enough arguments passed".to_string());
    }

    Ok(Config::new(args[1].clone(), args[2].clone()))
}

struct Config {
    query : String ,
    file_name  : String 
}
impl Config {
    fn new (query : String , file_name:String ) -> Config {
        Config {query , file_name}
    }
}
