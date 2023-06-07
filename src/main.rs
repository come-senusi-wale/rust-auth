use std::io::{Result};
use actix_web::{HttpServer, App, web};

mod scopes;
mod ectraction;
use scopes::user_scope;


#[actix_web::main]
async fn main() -> Result<()> {
    // dotenv().ok();

    // let config = crate::config::Config::from_env().unwrap();

    // let pool = config.pg.create_pool(NoTls).unwrap();

    HttpServer::new(move || ( 

        App::new()
            .app_data(web::Data::new(String::from("secret")))
            .service(user_scope())
    ))
    .bind(("127.0.0.1", 4000))?
    .run()
    .await
}
