#[derive(Serialize, Deserialize)]
pub struct AccessTokenResponnse {
    pub access_token: String,
    pub token_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct CodeResponnse {
    pub code: String,
    
}

#[derive(Serialize, Deserialize)]
pub struct ExtendedAccessTokenResponnse {
    pub access_token: String,
    pub machine_id: String,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct GetAlbumUrlRequest {
    pub access_token: String,
    pub token_type: String,
}




#[derive(RustcDecodable, RustcEncodable)]
pub struct Images {
    pub images: Vec<String>
}



#[derive(Serialize, Deserialize)]
pub struct CreateAlbumResponse {
    pub id: String
}


#[derive(Serialize, Deserialize)]
pub struct UploadPhotoResponse {
    pub id: String,
    pub post_id: String
}







#[derive(Serialize, Deserialize)]
pub struct FacebookPages {
    pub data: Vec<Datum>,
    pub paging: Paging,
}

#[derive(Serialize, Deserialize)]
pub struct Datum {
    pub access_token: String,
    pub category: String,
    pub category_list: Vec<CategoryList>,
    pub name: String,
    pub id: String,
    pub tasks: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub  struct CategoryList {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub  struct Paging {
    pub cursors: Cursors,
}

#[derive(Serialize, Deserialize)]
pub  struct Cursors {
    pub before: String,
    pub after: String,
}
