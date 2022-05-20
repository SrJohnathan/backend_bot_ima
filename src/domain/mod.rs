 pub mod models;

use reqwest::{Client};
use rocket::response::status;
use rocket::response::status::{BadRequest, Created};
use rocket::serde::json::Json;
use serde_derive::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct ResponseApi {
    id: i32,
    r#type: bool,
    jwt: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Auth {
    pub username: String,
    pub password: String,
}


impl Auth {
    pub fn new(username: String, password: String) -> Self { Auth { username, password } }

    pub async fn post(&self, path: String) -> Result<Created<Json<ResponseApi>>, BadRequest<String>> {
        let req = Client::new();


        let js = &serde_json::to_value(self).expect("Error convert object to json");
        let res = req.post(path).json(js)
            .send().await;

        match res {
            Ok(v) => {
                let js: ResponseApi = serde_json::from_str(
                    &v.text().await.expect("Error getting response from api"))
                    .expect("Error converting response api");

                Ok(status::Created::new("").body(Json(js)))
            }
            Err(e) => {
                Err(status::BadRequest(Some(String::from(e.to_string()))))
            }
        }
    }
}