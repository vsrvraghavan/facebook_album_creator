

extern crate reqwest;
use nickel::{JsonBody, Request, Response};
use app::models::{Images,  FacebookPages, Datum, CreateAlbumResponse, UploadPhotoResponse};
use std::collections::HashMap;
use serde_json;
use std::io::Read;

use app;


pub fn get_page_access_token(_request: &mut Request<'_, '_>, _response: &mut Response<'_>) -> String {

        let imgs = _request.json_as::<Images>().unwrap();
        //let mut tst = imgs.images;
        
        if imgs.images.capacity() < 2 {
                println!("minimum 2 images has to be passed ");                       
                //return response.send(format!("minimum 2 images has to be passed"));
                return "minimum 2 images has to be passed".to_owned();
        }

        let fb_details = app::util::globals::get_fb_hash_map();
        
         println!("{:?}", fb_details);

        //let mut fb_details = HASHMAP.lock().unwrap();  

        let  mut page_list_url = "https://graph.facebook.com/me/accounts?access_token=".to_owned();
        page_list_url.push_str(&*fb_details["access_token"]).to_owned();
       
        drop(fb_details);

        let mut res = reqwest::get(&page_list_url)        
            .expect("Failed to send request");
        
        /*
        println!("{}", res.status());
        for header in res.headers().iter() {
            println!("{}: {}", header.name(), header.value_string());
        }
        */
        let mut buf = String::new();
        res.read_to_string(&mut buf).expect("Failed to read response");
        
        

        let mut page_access_token = String::new();
        let  model: FacebookPages = serde_json::from_str(&buf).unwrap();
        print!("{} ", "\n 454545454545 \n");
        
        let  dat_vec: Vec<Datum> = model.data;

        for x in 0..dat_vec.len() {
            
            let data_vec = &dat_vec[x];
            let mut aid = &data_vec.id;
            let mut aname = &data_vec.name;
            let mut acctok = &data_vec.access_token;
            println!("PAGE ACCESS TOKEN  : {} {} {}", aid, aname, acctok);
            if aid.trim() == "252369582084953" {
                    page_access_token = acctok.to_owned();
                    break;
            }            
        }
    return page_access_token;
}



pub fn create_album(page_access_token: String) -> String{

    let mut map = HashMap::new();

    map.insert("name", "Raga Album Rust2");
    map.insert("message", "Album Made By Rust App");
    map.insert("access_token", &page_access_token);

    let album_create_url = "https://graph.facebook.com/v3.1/me/albums".to_owned();
    
    let client = reqwest::Client::new();
    let mut res = client.post(&album_create_url)
    .json(&map)
    .send()
    .expect("Failed to send request");

    /*
    println!("{}", res.status());
    for header in res.headers().iter() {
        println!("{}: {}", header.name(), header.value_string());
    }
    */
    let mut buf = String::new();
    res.read_to_string(&mut buf).expect("Failed to read response");

    let  model: CreateAlbumResponse = serde_json::from_str(&buf).unwrap();
    let album_id: String = model.id;
    println!("album_id {}", album_id);
    return album_id.to_string();

}


pub fn post_image_to_album(page_accesstoken: String, album_id: String, image: String) -> String{


    println!("url {}", image);
    println!("page_accesstoken {}", page_accesstoken);


    let mut map = HashMap::new();
    map.insert("url", image);
    map.insert("access_token", page_accesstoken);



    let mut post_image_to_album_url = "https://graph.facebook.com/v3.1/".to_owned();
    post_image_to_album_url.push_str(&album_id).to_owned();
    post_image_to_album_url.push_str("/photos").to_owned();


    let client = reqwest::Client::new();
    let mut res = client.post(&post_image_to_album_url)
    .json(&map)
    .send()
    .expect("Failed to send request");
    /*
    println!("{}", res.status());
    for header in res.headers().iter() {
        println!("{}: {}", header.name(), header.value_string());
    }
    */
    let mut buf = String::new();
    res.read_to_string(&mut buf).expect("Failed to read response");

    let  model: UploadPhotoResponse = serde_json::from_str(&buf).unwrap();
    let photo_id: String = model.id;
    
    println!("photo_id {}", &photo_id);


    return photo_id.to_string();

    
}
