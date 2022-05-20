use rocket::response::status::{BadRequest, Created};
use rocket::serde::json::{Json, Value};
use crate::header::{Admin, RequestApi};
use std::future::Future;
use reqwest::{Error, StatusCode};
use rocket::response::status;
use rocket_okapi::{openapi};
use rocket_okapi::okapi::schemars::{self, JsonSchema};
use serde::{Serialize, Deserialize};
use rocket::http::Status;
use crate::domain::models::*;
use crate::fcm::send_message_fcm;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, JsonSchema)]
pub struct Message {
    pub r#type: i32,
    pub text: Option<String>,
    pub originalUrl: Option<String>,
    pub previewUrl: Option<String>,
    pub url: Option<String>,
    pub apikey: String,
    pub mynumber: String,
    pub destination: String,
}

#[derive(Serialize, Deserialize, Clone, JsonSchema)]
pub struct MessageResponse {
    pub status: i32,
    pub message: Option<String>,
}


#[openapi(tag = "Whatsapp")]
#[post("/whatsapp/send/message", format = "application/json", data = "<task>")]
pub async fn message(task: Json<Message>) -> Result<Created<Json<MessageResponse>>, BadRequest<String>> {
    let message = task.0;

    let mut m = SendMessage::new(Channels::whatsapp, message.mynumber, message.apikey);

    let ty = match message.r#type {
        1 => "text",
        2 => "image",
        _ => { "" }
    };

    let mg = MessageGupshup {
        r#type: ty.to_string(),
        originalUrl: message.originalUrl
        ,
        previewUrl: message.previewUrl
        ,
        text: message.text
        ,
        url: message.url,
    };

    match m.send(mg, message.destination).await {
        Ok((c, s)) => {
            let mr = MessageResponse { status: c.as_u16() as i32, message: Option::Some(s) };
            Ok(status::Created::new("".to_string()).body(Json(mr)))
        }
        Err(e) => Err(BadRequest(Some(e.to_string())))
    }
}

pub enum Types {
    Enviado,
    Ler,
    Entregue,
    Falha,
    Enfileiramento,
}


#[openapi(tag = "Whatsapp")]
#[post("/whatsapp/send/wp", format = "application/json", data = "<task>")]
pub async fn webhook(cli:RequestApi<'_>,task: Json<serde_json::Value>) -> Status {
    let req = cli.0;
    let message = task.0;
    let d = message.get("type");
    match d {
        None => { println!("não encontrou type") }
        Some(c) => {
            if c.as_str().unwrap().eq("message-event") {
                let pl = message.get("payload").unwrap();
                let ty = pl.get("type").unwrap();

                if ty.as_str().unwrap().eq(&"enqueued".to_string()) {
                    let msg: ParentMessage<MessageEvent<Enqueued>> = serde_json::from_str(&message.to_string()).unwrap();
                } else if ty.as_str().unwrap().eq(&"failed".to_string()) {
                    let msg: ParentMessage<MessageEvent<Failed>> = serde_json::from_str(&message.to_string()).unwrap();
                } else if ty.as_str().unwrap().eq(&"sent".to_string()) {
                    let msg: ParentMessage<MessageEvent<Sent>> = serde_json::from_str(&message.to_string()).unwrap();
                } else if ty.as_str().unwrap().eq(&"delivered".to_string()) {
                    let msg: ParentMessage<MessageEvent<Delivered>> = serde_json::from_str(&message.to_string()).unwrap();
                } else if ty.as_str().unwrap().eq(&"read".to_string()) {
                    let msg: ParentMessage<MessageEvent<Read>> = serde_json::from_str(&message.to_string()).unwrap();
                } else {}
            } else if c.as_str().unwrap().eq("message") {
                let pl = message.get("payload").unwrap();
                let ty = pl.get("type").unwrap();

                if ty.as_str().unwrap().eq(&"text".to_string()) {
                    let msg: ParentMessage<MessageGP<Text>> = serde_json::from_str(&message.to_string()).unwrap();

                  let send =  send_message_fcm(&req,String::from("dGopRbaDEPBzMzjy5Xc9jF:APA91bF_aBYszj7cLOhvNyWtdg71SGopvu9x4EcYYLw0DxYwZjTTGRnXNxi4Dt0kgSCejCA1hZe_Zp8iBhVEy4jmBaefG5ukiuIJzGJ0dci9qRjLVWleuN7kdohLmN-1vkvK7uwd0I1C"),String::from("ola")).await;

                  match send {
                      Ok(c) => {  println!("{}",c.1) }
                      Err(e) => { println!("{}",e.to_string()) }
                  };

                } else if ty.as_str().unwrap().eq(&"image".to_string()) {
                    let msg: ParentMessage<MessageGP<Image>> = serde_json::from_str(&message.to_string()).unwrap();
                } else if ty.as_str().unwrap().eq(&"file".to_string()) {
                    let msg: ParentMessage<MessageGP<File>> = serde_json::from_str(&message.to_string()).unwrap();
                } else if ty.as_str().unwrap().eq(&"audio".to_string()) {
                    let msg: ParentMessage<MessageGP<Audio>> = serde_json::from_str(&message.to_string()).unwrap();
                } else if ty.as_str().unwrap().eq(&"video".to_string()) {
                    let msg: ParentMessage<MessageGP<Video>> = serde_json::from_str(&message.to_string()).unwrap();
                } else if ty.as_str().unwrap().eq(&"location".to_string()) {
                    let msg: ParentMessage<MessageGP<Location>> = serde_json::from_str(&message.to_string()).unwrap();
                } else if ty.as_str().unwrap().eq(&"button_reply".to_string()) {
                    let msg: ParentMessage<MessageGP<ButtonReply>> = serde_json::from_str(&message.to_string()).unwrap();
                } else if ty.as_str().unwrap().eq(&"list_reply".to_string()) {
                    let msg: ParentMessage<MessageGP<ListReply>> = serde_json::from_str(&message.to_string()).unwrap();
                } else {}
            }else {}
        }
    }


    Status::Ok
}
