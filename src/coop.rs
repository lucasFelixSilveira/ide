pub mod live;
pub mod utils;
pub mod routes;

use actix_web::{App, HttpServer};

#[actix_web::main]
pub async  fn start() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(routes::root)
            .service(routes::shutdown)
            .service(routes::new_user)
            .service(routes::buffer_content)
            .service(routes::values)
    })
    .bind("127.0.0.1:6932")?
    .run()
    .await
}