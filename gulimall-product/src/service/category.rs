use actix_web::web::Json;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, UpdateResult};
use sea_orm::ActiveValue::Set;
use crate::entity::{pms_category, pms_category::Entity  as Category};

pub struct PmsCategoryService;
impl PmsCategoryService {
     pub async fn list_with_tree(db: &DatabaseConnection, ) -> Result<Vec<pms_category::Model>, DbErr> {
          // 查出所有的分类
          let category_entities = Category::find().all(db).await?;
          // 组装成父子树形结构, 找到所有1级分类
          let mut level1_menus = category_entities.iter().filter(|category| {
               category.parent_cid == Some(0)
          }).map(|menu| {
               let mut menu = menu.clone();
               menu.children = get_childrens(&menu, &category_entities);
               menu
          }).collect::<Vec<pms_category::Model>>();
          level1_menus.sort_by(|a, b| {
               Some(a.sort).cmp(&Some(b.sort))
          });
          Ok(level1_menus)
     }

     pub async fn save(db: &DatabaseConnection, form_data: Json<pms_category::Model>,) ->Result<pms_category::Model ,DbErr>{
          let json = serde_json::to_value(form_data).expect("TODO: panic message");
          let new_category = pms_category::ActiveModel::from_json(json)?;

          new_category.insert(db).await
     }
     pub async fn delete(db: &DatabaseConnection,cate_id: i64) ->Result<(),DbErr>{
          let delete_status: Option<i8> = Some(0);


          let category = Category::find_by_id(cate_id).one(db).await?;

          let mut category:pms_category::ActiveModel = category.unwrap().into();
          category.show_status = Set(delete_status);
          category.update(db).await?;
          Ok(())
     }
     pub async fn find_by_id(db: &DatabaseConnection,cate_id: i64) -> Result<pms_category::Model,DbErr>{
          let category = Category::find_by_id(cate_id).one(db).await?;
          match category {
               Some(category) => Ok(category.into()),
               None => Err(DbErr::Json("can find this cate_id".to_string()))
          }
     }
     // 批量修改
     pub fn updateBatchById (db: &DatabaseConnection,form_data_vec: Vec<pms_category::Model>){
          // form_data_vec.iter().map(|category|{
          //
          // })
          // let update_result: UpdateResult = Category::update_many()
          todo!()
     }
     // 要开启事务
     pub async fn update_cascade(db: &DatabaseConnection, form_data: pms_category::Model ) {
          // let new_category = pms_category::ActiveModel{
          //      cat_id:Set(form_data.cat_id),
          //      name: Set(form_data.name),
          //      icon:Set(form_data.icon),
          //      product_unit : Set(form_data.product_unit),
          //      ..Default::default()
          // };
          // pms_category::ActiveModel::from_json(form_data)
          // new_category.update(db).await?;
          // if form_data.name.is_some() {
          //
          // }
          todo!()
     }
}
fn get_childrens(menu: &pms_category::Model, category_entities: &Vec<pms_category::Model>) -> Vec<pms_category::Model> {
     let mut menu1 =category_entities.iter().filter(|category| {
          category.parent_cid == Some(menu.cat_id)
     }).map(|category| {
          let mut category = category.clone();
          category.children= get_childrens(&category, category_entities);
          category
     }).collect::<Vec<pms_category::Model>>();
     menu1.sort_by(|a,b| {
          Some(a.sort).cmp(&Some(b.sort))
     });
     menu1
}