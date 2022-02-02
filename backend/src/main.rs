use crate::handler::draw_card::draw_card;
use crate::handler::join_game::join_game;
use crate::handler::lb_reconnect::lb_reconnect;
use crate::handler::restart_game::start_game;
use crate::handler::service::auth::AuthService;
use crate::handler::{create_game::create_game, service::lb_connector::LoadBalancerConnector};
use crate::repo::game_repo::InMemoryGameRepo;
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use clap::Parser;
use handler::{play_card::play_card, ws_connect::ws_connect};
use std::{env, sync::Mutex};

mod cards;
mod err;
mod gamestate;
mod handler;
mod repo;
mod ws;

#[derive(Parser)]
#[clap(version = "1.0", author = "L.G.")]
struct Opts {
    #[clap(short = 'p', long = "port", default_value = "9000")]
    port: String,
    #[clap(short = 'l', long = "lbaddr", default_value = "rust-uno.herokuapp.com")]
    load_balancer_addr: String,
    #[clap(short = 's', long = "servername")]
    server_addr: String,
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    let port = match env::var("PORT") {
        Ok(p) => p,
        Err(_) => opts.port,
    };

    let game_repo = web::Data::new(Mutex::new(InMemoryGameRepo::new()));
    let auth_service = web::Data::new(AuthService::new());

    let lb_connector = LoadBalancerConnector::new(opts.load_balancer_addr, opts.server_addr);
    if lb_connector.connect().await.is_err() {}
    let lb_connector = web::Data::new(lb_connector);

    println!("Starting server on port {}", port);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_header()
            .allow_any_origin()
            .allow_any_method();

        App::new()
            .wrap(cors)
            .app_data(game_repo.clone())
            .app_data(auth_service.clone())
            .app_data(lb_connector.clone())
            .service(create_game)
            .service(start_game)
            .service(draw_card)
            .service(join_game)
            .service(play_card)
            .service(ws_connect)
            .service(lb_reconnect)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await?;

    Ok(())
}
