use rocket::post;
use rocket::serde::json::Json;

use crate::Error;

use super::recv::RecvInfo;
use super::reply::ReplyInfo;


#[post("/api/search", format = "application/json", data = "<data>")]
pub async fn search(data: Json<RecvInfo>) -> Result<Json<ReplyInfo>, Error> {
    let inner = data.into_inner();
    let reply_info = inner.to_reply_info().await?;
    Ok(Json(reply_info))
}