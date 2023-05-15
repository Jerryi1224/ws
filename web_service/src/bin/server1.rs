use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::io;
use actix_web::web::get;

// 配置route
pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", get().to(health_check_handler));
}

//配置 handler
pub async fn health_check_handler() -> impl Responder {
    HttpResponse::Ok().json("Actix Web Service is running!")
}

// 实例化HTTP server 并运行

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let app = move || App::new().configure(general_routes);
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}