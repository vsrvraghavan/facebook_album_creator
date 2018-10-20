
use std;
use std::error::Error;
use std::time::Duration;

use job_scheduler::{JobScheduler, Job};

use app;

extern crate reqwest;

use app::models::{AccessTokenResponnse, CodeResponnse, ExtendedAccessTokenResponnse};



pub fn run(cron_expression: String, access_token: String, app_id: String, app_secret: String, redirect_uri: String, page_id: String, page_name: String) -> Result<bool,Box<Error>> {
    


    let _map = app::util::globals::put_fb_hash_map(access_token, app_id, app_secret, redirect_uri, page_id, page_name);
    

    let mut sched = JobScheduler::new();    
    sched.add(Job::new(cron_expression.parse().unwrap(), || {              
        //get_extended_access_token(map.to_owned());     
        get_extended_access_token();
    }));    
    loop {
        sched.tick();
        std::thread::sleep(Duration::from_millis(500));
    }    
}


fn get_extended_access_token() {

    println!();
    println!();
    println!();
    println!();
    let client = reqwest::Client::new();
    // Get Short Lived App Access Token from user access token and AppID and App Secret    
    // Aquire Lock
    //let mut fb_details = HASHMAP.lock().unwrap();
    let fb_details = app::util::globals::get_fb_hash_map();

    println!("updated access_token  = {:?}", fb_details["access_token"]);   

    let mut page_access_tok_request_url = "https://graph.facebook.com/oauth/access_token?client_id=".to_owned();
    page_access_tok_request_url.push_str(&*fb_details["app_id"]).to_owned();
    page_access_tok_request_url.push_str("&client_secret=").to_owned();
    page_access_tok_request_url.push_str(&*fb_details["app_secret"]).to_owned();
    page_access_tok_request_url.push_str("&redirect_uri=").to_owned();
    page_access_tok_request_url.push_str(&*fb_details["redirect_url"]).to_owned();
    page_access_tok_request_url.push_str("&grant_type=fb_exchange_token&fb_exchange_token=").to_owned();
    page_access_tok_request_url.push_str(&*fb_details["access_token"]).to_owned();


    let mut response = client.get(&*page_access_tok_request_url)
    .send()
    .expect("Failed to send request");
    let mut temp_access_token = String::new();
    if let Ok(access_token_data) = response.json::<AccessTokenResponnse>() {        
        temp_access_token = access_token_data.access_token;
    }
    
    // Get code from previously generated Short Lived Access Token 
    let  temp_access_token_str: &str = &*temp_access_token;  
    let mut get_code_request_url = "https://graph.facebook.com/oauth/client_code?access_token=".to_owned();
    get_code_request_url.push_str(temp_access_token_str);
    get_code_request_url.push_str("&client_id=").to_owned();
    get_code_request_url.push_str(&*fb_details["app_id"]).to_owned();
    get_code_request_url.push_str("&client_secret=").to_owned();
    get_code_request_url.push_str(&*fb_details["app_secret"]).to_owned();
    get_code_request_url.push_str("&redirect_uri=").to_owned();
    get_code_request_url.push_str(&*fb_details["redirect_url"]).to_owned();

    response = client.get(&*get_code_request_url)
    .send()
    .expect("Failed to send request");
    let mut access_code = String::new();
    if let Ok(access_code_data) = response.json::<CodeResponnse>() {        
        access_code = access_code_data.code;
    }


    // Get Extended Access Token from previously generated Code

    let mut get_extended_access_token_request_url = "https://graph.facebook.com/oauth/access_token?code=".to_owned();
    get_extended_access_token_request_url.push_str(&access_code);
    get_extended_access_token_request_url.push_str("&client_id=").to_owned();
    get_extended_access_token_request_url.push_str(&*fb_details["app_id"]).to_owned();
    get_extended_access_token_request_url.push_str("&redirect_uri=").to_owned();
    get_extended_access_token_request_url.push_str(&*fb_details["redirect_url"]).to_owned();

    response = client.get(&*get_extended_access_token_request_url)
    .send()
    .expect("Failed to send request");

    //let mut extended_access_token = String::new();
    
    if let Ok(extemnded_access_token_data) = response.json::<ExtendedAccessTokenResponnse>() {    
        let _dtls = app::util::globals::put_fb_hash_map(extemnded_access_token_data.access_token, fb_details["app_id"].to_owned(), fb_details["app_secret"].to_owned(), fb_details["redirect_url"].to_owned(), fb_details["page_id"].to_owned(), fb_details["page_name"].to_owned());

    }
    
            //let date = Local::now();
            //println!("{}", date.format("%Y-%m-%d][%H:%M:%S"));
            println!("extended access_token  = {:?}", fb_details["access_token"]);   
            // Release Lock
            drop(fb_details);
                                           
}