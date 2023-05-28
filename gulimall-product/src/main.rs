use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use sea_orm::{Database, DatabaseConnection};
use gulimall_product::AppState;
use gulimall_product::router::{delete, info, list, list_tree, save};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db: DatabaseConnection = Database::connect("mysql://root:root@localhost:3306/gulimall_pms").await.expect("Failed to connect to database");
    let state = AppState { db };
    HttpServer::new(move || {
        App::new()
            .service(info)

            .app_data(web::Data::new(state.clone()))
            .configure(init)
    })

        .bind(("127.0.0.1", 8090))?
        .run()
        .await
}
fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(list);
    cfg.service(list_tree);
    cfg.service(save);
    cfg.service(delete);
}
