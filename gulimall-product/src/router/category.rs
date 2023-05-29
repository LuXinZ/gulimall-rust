use actix_web::{get,  web, App, HttpResponse, HttpServer, Responder,post};
use actix_web::web::to;

use serde::Serialize;

use crate::AppState;
use crate::entity::pms_category;
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
    match list_vec {
        Ok(l) => Ok(MessageResponse::new(Some(serde_json::to_value(&l).unwrap()))),
        Err(e) => Err(AppError::DbError(e))
    }
}

#[get("/info/{cate_id}")]
pub async fn info(cate_id: web::Path<u32>) ->  AppResult {
  handle_response(give_err(cate_id.into_inner()))

}
fn give_err (id:u32) ->Result<String,AppError>{
    if id > 0 {
        Ok("Success".into())
    }else{
        Err(AppError::Conflict("bux".into()))
    }
}
#[post("/save")]
pub async fn save(data: web::Data<AppState>, post_form: web::Json<Model>,) -> AppResult{
    let db = &data.db;
    let form = post_form.into_inner();
    let res = PmsCategoryService::save(db,form).await;
    handle_response(res)
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