use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use log::{LevelFilter, info};
use settings::Settings;
use simple_logger::SimpleLogger;
use sqlx::mysql::MySqlPoolOptions;
use tera::{Context, Tera};

mod api;
mod settings;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error: {}", e);
                ::std::process::exit(1);
            }
        };

        tera
    };
}

#[get("/")]
async fn index() -> impl Responder {
    let content = "Generate a tiny code for url";
    let mut data = Context::new();

    data.insert("content", content);

    let rendered = TEMPLATES.render("index.html", &data).unwrap();

    HttpResponse::Ok().body(rendered)
}

#[actix_web::main]
async fn main() -> Result<(), sqlx::Error> {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

    let s = Settings::new().unwrap();
    let ip = s.server.get_ip();
    let url = s.database.url;
    let pool_size = s.database.pool_size;

    let pool = MySqlPoolOptions::new()
        .max_connections(pool_size)
        .connect(&url)
        .await?;

    info!("server listening at http://{:?}", &ip);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(index)
            .service(api::links::create_link)
            .service(api::links::get_all_links)
            .service(api::links::get_from_link)
    })
    // .bind(("127.0.0.1", 8080))?
    .bind(&ip)?
    .run()
    .await?;

    Ok(())
}
