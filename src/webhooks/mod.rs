



mod whats_gupshup;

#[post("/auth/text", format = "application/json", data = "<task>")]
pub async fn login(task: Json<Auth>) -> Result<Created<Json<ResponseApi>>, BadRequest<String>> {

    let js = task.0;
    let auth = Auth::new(js.username,js.password);
    auth.post(String::from("")).await

}