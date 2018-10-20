


use std::sync::Mutex;
use std::collections::HashMap;

//extern crate chrono;
//use chrono::Local;



lazy_static! {
    pub static ref HASHMAP: Mutex<HashMap<&'static str, String>> = {
        let mut m = HashMap::new();
        m.insert("access_token", "EAADfeoMn6lIBAHBJSFQcD6uo4pdX3ljQSSppj6ZC0CmOEAMSJ9FKABGWmRd6bkZCLOHy7WZBThqMgxr50ql3ZBO2HnvwuAd29ALt0M7H0IIdb1jhsd95HOMHSkejFwXvbcRdMgxqX4sJVdPlAafglMSkMfd3Xo0ZD".to_owned());
        m.insert("app_id", "245717279435346".to_owned());
        m.insert("app_secret", "24662e62fde265293ecf8bbdb799c094".to_owned());
        m.insert("redirect_url", "http://localhost/upload_fb_images.html".to_owned());
        m.insert("page_id", "252369582084953".to_owned());
        m.insert("page_name", "Sahre-Album".to_owned());
        Mutex::new(m)
    };    
}




pub fn get_fb_hash_map() -> HashMap<&'static str, String> {
        let mut hashmap = HashMap::new();
        let map = HASHMAP.lock().unwrap();  

        hashmap.insert("access_token", map["access_token"].clone());
        hashmap.insert("app_id", map["app_id"].clone());
        hashmap.insert("app_secret", map["app_secret"].clone());
        hashmap.insert("redirect_url", map["redirect_url"].clone());
        hashmap.insert("page_id", map["page_id"].clone());
        hashmap.insert("page_name", map["page_name"].clone());
        // Release Lock
        drop(map);
        return hashmap;        
}


pub fn put_fb_hash_map(access_token: String , app_id: String , app_secret: String , redirect_url: String, page_id: String, page_name: String) -> HashMap<&'static str, String>  {
        let mut _hashmap = HashMap::new();
        let mut map = HASHMAP.lock().unwrap();    
        map.insert("access_token", access_token);
        map.insert("app_id", app_id);
        map.insert("app_secret", app_secret);
        map.insert("redirect_url", redirect_url);
        map.insert("page_id", page_id);
        map.insert("page_name", page_name);
        // Release Lock
        _hashmap = map.clone();        
        drop(map);
        return _hashmap;        
}