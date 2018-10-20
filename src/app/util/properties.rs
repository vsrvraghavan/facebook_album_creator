

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::iter::FromIterator;
use std::path::Path;

pub fn config_from_file(path: String) -> Config {
    let path = Path::new(&path);
    let file = File::open(path).expect("File not found or cannot be opened");
    let content = BufReader::new(&file);
    let mut cfg = Config::new();
 
    for line in content.lines() {
        let line = line.expect("Could not read the line");
        // Remove whitespaces at the beginning and end
        let line = line.trim();

        // Ignore comments and empty lines
        if line.starts_with("#") || line.starts_with(";") || line.is_empty() {
            continue;
        }
 
        // Split line into parameter name and rest tokens
        let tokens = Vec::from_iter(line.split_whitespace()); 
        let name = tokens.first().unwrap();
        let tokens = tokens.get(1..).unwrap();
         
        // Remove the equal signs
        let tokens = tokens.iter().filter(|t| !t.starts_with("="));
        
        // Remove comment after the parameters
        let tokens = tokens.take_while(|t| !t.starts_with("#") && !t.starts_with(";"));
        
        // Concat back the parameters into one string to split for separated parameters
        let mut parameters = String::new();
        tokens.for_each(|t| { parameters.push_str(t); parameters.push(' '); println!("t : {}",t); });
        // Splits the parameters and trims
        let parameters = parameters.split(',').map(|s| s.trim());
        
        // Converts them from Vec<&str> into Vec<String>
        let parameters: Vec<String> = parameters.map(|s| s.to_string()).collect();
        
        // Setting the config parameters
        match name.to_lowercase().as_str() {
            "app_id" => cfg.app_id = parameters.get(0).cloned().unwrap(),
            "app_secret" => cfg.app_secret = parameters.get(0).cloned().unwrap(),
            "app_access_token" => cfg.app_access_token = parameters.get(0).cloned().unwrap(),
            "app_redirect_url" => cfg.app_redirect_url = parameters.get(0).cloned().unwrap(),
            "page_id" => cfg.page_id = parameters.get(0).cloned().unwrap(),
            "page_name" => cfg.page_name = parameters.get(0).cloned().unwrap(),
            _ => (),
        }
    }
 
    cfg
}
 
#[derive(Clone, Debug)]
pub struct Config {
    pub app_id: String,
    pub app_secret: String,
    pub app_access_token: String,
    pub app_redirect_url: String,
    pub page_id: String,
    pub page_name: String,
}
 
impl Config {
    fn new() -> Config {
        Config {
            app_id: "".to_owned(),
            app_secret: "".to_owned(),
            app_access_token: "".to_owned(),
            app_redirect_url: "".to_owned(),
            page_id: "".to_owned(),
            page_name: "".to_owned(),
        }
    }
}