use actix_web::{get,  web, App, HttpResponse, HttpServer, Responder,post};

use crate::AppState;
use crate::entity::pms_category;
use crate::service::PmsCategoryService;

// 列表
#[get("/list")]
pub async fn list(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
//  三级分类
#[get("/list/tree")]
pub async fn list_tree(data: web::Data<AppState>) ->  impl Responder {

    let db = &data.db;
    let list_vec = PmsCategoryService::list_with_tree(db).await.expect("list_with_tree");
    web::Json(list_vec)
}

#[get("/info/{cate_id}")]
pub async fn info(cate_id: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello world! :{}", cate_id.to_string()))
}
#[post("/save")]
pub async fn save(data: web::Data<AppState>, post_form: web::Json<pms_category::Model>,) -> impl Responder {
    let db = &data.db;
    let form = post_form.into_inner();
    PmsCategoryService::save(db,form).await.expect("save failed");
    HttpResponse::Ok().body("success")
}
#[get("/update")]
pub async fn update() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
#[get("/update_nodes")]
pub async fn update_nodes() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
#[get("/delete/{cate_id}")]
pub async fn delete(data: web::Data<AppState>,cate_id : web::Path<i64>) -> impl Responder {
    let db = &data.db;
    let cate_id = cate_id.into_inner();
   PmsCategoryService::delete(db,cate_id).await.expect("update failed");
    HttpResponse::Ok().body("success")
}