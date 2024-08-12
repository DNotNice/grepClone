use std::fs;
use std::error::Error;
use std::env;

pub fn run(config : Config) -> Result<() , Box< dyn Error>> {
    let content = fs::read_to_string(config.file_name)?;
    println!("reading query {} " ,config.query );

    let res = if config.case_sensitive {
        search(&config.query, &content)
    }else {
      search_case_insensitive(&config.query, &content)
    };
    for line in res {
        println!("{}" , line);
    }

    Ok(())
}
 
pub struct Config {
    pub query : String ,
    pub file_name  : String ,
    pub case_sensitive : bool
}
impl Config {
    pub fn new(args : &Vec<String>) -> Result<Config , String>{
        if args.len() < 3 {
          return Err("not enough arguments passed".to_string());
        }
           let query = args[1].clone();
           let file_name = args[2].clone(); 
           let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
           println!("{case_sensitive}");
        Ok(Config {query , file_name , case_sensitive})
    }
}

pub fn search<'a>(query : &str, contents : &'a str) -> Vec<&'a str>{
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.contains(query){
            result.push(line);
        }
    }
    result
}
pub fn  search_case_insensitive<'a>(query :&str , contents : &'a str)-> Vec<&'a str>{
    let mut result = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query){
            result.push(line);
        }
    }
    result
}


#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn case_sensitive(){
        let query = "Duct";
        let contents = "\
Rust:
safe, fast and reliable.
Pick Three.
Duct Tape.";
   
        assert_eq!(vec!["Duct Tape."] , search(query , contents));
    }

    #[test]
    fn one_result(){
        let query = "Us";
        let contents = "\
Rust:
safe, fast and reliable.
Trust me, Pick Three.";
   
        assert_eq!(vec!["Rust:", "Trust me, Pick Three."] ,
      search_case_insensitive(query , contents));
    }

}
 