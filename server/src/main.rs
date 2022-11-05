use actix_web::{App, get, HttpResponse, HttpServer, middleware, Responder};

mod app_config;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = app_config::load_config();

    env_logger::Builder::new()
        .parse_filters(&config.logger.filters)
        .parse_write_style(&config.logger.write_style)
        .init();

    log::info!("Starting server at {}:{}", config.app.host, config.app.port);

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(hello)
    })
        .bind((config.app.host, config.app.port))?
        .run()
        .await
}
