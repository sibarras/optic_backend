use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use tokio_postgres::NoTls;

mod config;
mod db;
mod errors;
mod handlers;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = crate::config::Config::from_env().unwrap();
    let pool = config.pg.create_pool(None, NoTls).unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    // .allowed_origin("http://localhost:8000")
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3000),
            )
            .app_data(web::Data::new(pool.clone()))
            .service(web::resource("/users").route(web::post().to(handlers::users::add_user)))
            .service(
                web::resource("/sales")
                    .route(web::get().to(handlers::sales::get_sales))
                    .route(web::post().to(handlers::sales::add_sale)),
            )
            .service(
                web::resource("/sales/{id}")
                    .route(web::get().to(handlers::sales::get_sale))
                    .route(web::delete().to(handlers::sales::delete_sale))
                    .route(web::put().to(handlers::sales::update_sale)),
            )
            .service(
                web::resource("/frame_brands")
                    .route(web::get().to(handlers::frame_brands::get_frame_brands))
                    .route(web::post().to(handlers::frame_brands::add_frame_brand)),
            )
            .service(
                web::resource("/frame_brands/{id}")
                    .route(web::get().to(handlers::frame_brands::get_frame_brand))
                    .route(web::delete().to(handlers::frame_brands::delete_frame_brand))
                    .route(web::put().to(handlers::frame_brands::update_frame_brand)),
            )
            .service(
                web::resource("/sale_kinds")
                    .route(web::get().to(handlers::sale_kinds::get_sale_kinds)),
            )
            .service(
                web::resource("/sale_kinds/{id}")
                    .route(web::get().to(handlers::sale_kinds::get_sale_kind)),
            )
    })
    .bind(config.server_addr.clone())?
    .run();
    println!("Server running at http://{}/", config.server_addr);

    server.await
}

// curl -i -d '{"saletype_id": "1", "framebrand_id": "1", "quantity": "1"}' -H 'Content-Type: application/json' http://127.0.0.1:8080/sales
