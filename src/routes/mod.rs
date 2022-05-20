use rocket::response::status::{BadRequest, Created};
use rocket::serde::json::Json;
use crate::domain::{Auth, ResponseApi};




#[post("/auth/text", format = "application/json", data = "<task>")]
pub async fn login(task: Json<Auth>) -> Result<Created<Json<ResponseApi>>, BadRequest<String>> {

    let js = task.0;
    let auth = Auth::new(js.username,js.password);
    auth.post(String::from("")).await

}