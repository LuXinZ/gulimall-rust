
use sea_orm::{DatabaseConnection, DbErr, EntityTrait};
use crate::entity::pms_category;
use crate::entity::prelude::PmsCategory;

pub async fn list_with_tree(db: &DatabaseConnection,)->Result<Vec<pms_category::Model>, DbErr>{
     // 查出所有的分类
     let category_entities = PmsCategory::find().all(db).await?;
     // 组装成父子树形结构, 找到所有1级分类
     let mut level1_menus = category_entities.iter().filter(|category|{
          category.parent_cid  == Some(0)
     }).map(|menu|{
          let mut menu = menu.clone();
          menu.children = get_childrens(&menu,&category_entities);
          menu
     }).collect::<Vec<pms_category::Model>>();
     level1_menus.sort_by(|a,b|{
          Some(a.sort).cmp(&Some(b.sort))
     });
     Ok(level1_menus)
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