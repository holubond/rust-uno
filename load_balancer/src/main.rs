use actix_cors::Cors;
use actix_web::{web, App, HttpServer, Error};
use clap::Parser;
use std::{
    env,
};

use crate::{game_server_repo::GameServerRepo, handler_register::register_game_server};

mod game_server_repo;
mod handler_register;

#[derive(Parser)]
#[clap(version = "1.0", author = "Ondrej Holub")]
struct Opts {
    #[clap(short = 'p', long = "port", default_value = "9900")]
    port: String,
}

async fn fallback_to_spa() -> actix_files::NamedFile {
    actix_files::NamedFile::open("./static/index.html").unwrap()
}

#[actix_web::main]
async fn main() -> Result<(), Error> {
    let port = match env::var("PORT") {
        Ok(p) => p,
        Err(_) => Opts::parse().port,
    };

    println!("Starting server on port {}", port);

    let game_server_repo = web::Data::new(GameServerRepo::new());

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_header()
            .allow_any_origin()
            .allow_any_method();

        App::new()
            .wrap(cors)
            .app_data(game_server_repo.clone())
            .service(register_game_server)
            .service(actix_files::Files::new("/", "./static").index_file("index.html"))
            .default_service(web::resource("").route(web::get().to(fallback_to_spa)))
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await?;

    Ok(())
}
