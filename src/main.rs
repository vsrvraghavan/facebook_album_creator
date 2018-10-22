//#![feature(nll)]
//#![allow(unused_variables)]
//#![feature(const_string_new)]
#![feature(type_ascription)]

pub mod app;
extern crate hyper;
extern crate rustc_serialize;
//use rustc_serialize::json::{self, Json, ToJson};
use rustc_serialize::json::{self};


extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
//use serde_json::{Value};
//use generated_module::[object Object];


#[macro_use]
extern crate nickel;
use nickel::{JsonBody,Nickel, HttpRouter, MediaType};



#[macro_use]
extern crate lazy_static;
extern crate reqwest;
extern crate job_scheduler;

use app::models::{Images};
use std::collections::HashMap;

use std::env;


fn main() {  
    let args: Vec<String> = env::args().collect();
    let mut path = String::from("/tmp/Usefile.conf");
    if args.len() > 1 {        
        path = args[1].to_owned();
        println!("config file arguments : {}", path);
    }else{
        println!("config file path missing in arguments \n example : fb_album_creator /etc/fb_details.properties");
    }

    //let path = String::from("/tmp/Usefile.conf");
    let cfg = app::util::properties::config_from_file(path);
 
    let mut server = Nickel::new();
    let mut router = Nickel::router();

     

    router.post("/create_get_album_url", middleware! { |_request, mut response|        
        
        let imgs = _request.json_as::<Images>().unwrap();
        print!("{} ", "\n Test get_page_access_token \n");
        let mut _page_access_token: String = app::util::httprequest::get_page_access_token(_request, &mut response);
        let mut page_access_token_clone = _page_access_token.clone();
        
        let mut  _album_id: String = app::util::httprequest::create_album(page_access_token_clone);
        page_access_token_clone = _page_access_token.clone();        
        let mut album_id_clone = _album_id.clone();        
        
        let mut _photo_id: String = app::util::httprequest::post_image_to_album(page_access_token_clone, album_id_clone, imgs.images[0].to_owned());
        page_access_token_clone = _page_access_token.clone();        
        album_id_clone = _album_id.clone(); 

        _photo_id: String = app::util::httprequest::post_image_to_album(page_access_token_clone, album_id_clone, imgs.images[1].to_owned());           
        album_id_clone = _album_id.clone(); 


        let flbtls = app::util::globals::get_fb_hash_map();

        /*
        let mut album_url = "https://www.facebook.com/pg/".to_owned();        
        album_url.push_str(&*flbtls["page_name"]).to_owned();        
        album_url.push_str("-").to_owned();
        album_url.push_str(&*flbtls["page_id"]).to_owned();
        album_url.push_str("/photos/?tab=album&album_id=").to_owned();
        album_url.push_str(&album_id_clone).to_owned();//  256758501646061
        */
        //https://www.facebook.com/pg/Sahre-Album-252369582084953/photos/?tab=album&album_id=266239830697928
        let mut album_url = "https://www.facebook.com/pg/".to_owned();        
        album_url.push_str(&*flbtls["page_name"]).to_owned();  
        album_url.push_str("-").to_owned();     
        album_url.push_str(&album_id_clone).to_owned();        
        album_url.push_str("/photos/?tab=album&album_id=").to_owned();
        //album_url.push_str(&*flbtls["page_id"]).to_owned();
        //album_url.push_str("/photos/?tab=album&album_id=").to_owned();
        album_url.push_str(&album_id_clone).to_owned();//  256758501646061


	println!("album URL {}", &album_url);


        let mut map = HashMap::new();
        map.insert("album_url", &album_url);
        // Set the returned type as JSON
        response.set(MediaType::Json);
        // Send back the result

        format!("The FB Details JSON is: {:?}", json::encode(&map).unwrap())
    });



    server.utilize(router);
    let _out = server.listen("127.0.0.1:9000");        
    //sched.add(Job::new("1/10 * * * * *".parse().unwrap(), || {        
    let  cron_expression = "0 0/59 * * * *".to_owned();
    //let  cron_expression = "1/10 * * * * *".to_owned();

    println!("server started :");
    println!("cfg.app_access_token: {}", cfg.app_access_token);

    let _result = app::scheduled_task::task::run(cron_expression, cfg.app_access_token, cfg.app_id, cfg.app_secret, cfg.app_redirect_url, cfg.page_id, cfg.page_name);    
}
