use actix_web::{get,  web, App, HttpResponse, HttpServer, Responder};

use crate::AppState;
use crate::service::list_with_tree;

//  三级分类
#[get("/list/tree")]
async fn list_tree() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
// 列表
#[get("/list")]
pub async fn list(data: web::Data<AppState>) ->  impl Responder {
    let db = &data.db;
    let list = list_with_tree(db).await.expect("list_with_tree");
    web::Json(list)
}

#[get("/info/{cate_id}")]
pub async fn info(cate_id: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello world! :{}", cate_id.to_string()))
}
#[get("/save")]
pub async fn save() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
#[get("/update")]
pub async fn update() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
#[get("/update_nodes")]
pub async fn update_nodes() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
#[get("/delete")]
pub async fn delete() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}