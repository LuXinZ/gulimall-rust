
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait};
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

     pub async fn save(db: &DatabaseConnection, form_data: pms_category::Model,) ->Result<pms_category::Model ,DbErr>{
          let new_category = pms_category::ActiveModel{
               name: Set(form_data.name),
               parent_cid: Set(form_data.parent_cid),
               cat_level: Set(form_data.cat_level),
               show_status: Set(form_data.show_status),
               sort: Set(form_data.sort),
               ..Default::default()
          };
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