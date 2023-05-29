use actix_web::{get,  web, App, HttpResponse, HttpServer, Responder,post};

use serde::Serialize;

use crate::AppState;
use crate::entity::pms_category::Model;
use crate::error::{AppError, AppResult, MessageResponse};
use crate::service::PmsCategoryService;
// 列表
#[get("/list")]
pub async fn list(data: web::Data<AppState>) -> AppResult {

    Ok(MessageResponse::new(None))
}
//  三级分类
#[get("/list/tree")]
pub async fn list_tree(data: web::Data<AppState>) ->  AppResult {
    let db = &data.db;
    let list_vec = PmsCategoryService::list_with_tree(db).await;
    handle_response(list_vec)
}

#[get("/info/{cate_id}")]
pub async fn info(data: web::Data<AppState>,cate_id: web::Path<i64>) ->  AppResult {
    let one_cate_info = PmsCategoryService::find_by_id(&data.db,cate_id.into_inner()).await;
    handle_response(one_cate_info)

}
#[post("/save")]
pub async fn save(data: web::Data<AppState>, post_form: web::Json<Model>,) -> AppResult{
    let db = &data.db;
    let res = PmsCategoryService::save(db,post_form).await;
    handle_response(res)
}
#[get("/update")]
pub async fn update() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
#[post("/update_nodes")]
pub async fn update_nodes(data: web::Data<AppState>, post_form: web::Json<Vec<Model>>,) -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
#[get("/delete/{cate_id}")]
pub async fn delete(data: web::Data<AppState>,cate_id : web::Path<i64>) -> impl Responder {
    let db = &data.db;
    let cate_id = cate_id.into_inner();
   PmsCategoryService::delete(db,cate_id).await.expect("update failed");
    HttpResponse::Ok().body("success")
}
fn handle_response<T, E>(
    res: Result<T, E>,
) -> AppResult where
    T: Serialize,
    E: std::error::Error + 'static,
    AppError: From<E>, {
    match res {
        Ok(l) => Ok(MessageResponse::new(Some(serde_json::to_value(&l).unwrap()))),
        Err(e) => Err(e.into())
    }
}