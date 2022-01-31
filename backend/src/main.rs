use crate::handler::create_game::create_game;
use crate::handler::draw_card::draw_card;
use crate::handler::join_game::join_game;
use crate::handler::restart_game::start_game;
use crate::handler::service::auth::AuthService;
use crate::repo::game_repo::InMemoryGameRepo;
use actix_cors::Cors;
use actix_web::{web, App, HttpServer, client::Client, http::StatusCode};
use clap::Parser;
use handler::{play_card::play_card, ws_connect::ws_connect};
use serde::Serialize;
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
    #[clap(short = 'l', long = "lbaddr", default_value = "http://rust-uno.herokuapp.com")]
    load_balancer_addr: String,
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    let (port, my_listening_port) = match env::var("PORT") {
        Ok(p) => (p, "443".to_string()),
        Err(_) => (opts.port.clone(), opts.port),
    };

    let game_repo = web::Data::new(Mutex::new(InMemoryGameRepo::new()));
    let auth_service = web::Data::new(AuthService::new());

    connect_to_load_balancer(opts.load_balancer_addr, my_listening_port).await;

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
            .service(create_game)
            .service(start_game)
            .service(draw_card)
            .service(join_game)
            .service(play_card)
            .service(ws_connect)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await?;

    Ok(())
}

#[derive(Serialize, Debug)]
pub struct RequestBody {
    port: String,
}

async fn connect_to_load_balancer(url: String, port: String) {
    let client = Client::default();

    let url = format!("{}/gameServer", url);

    println!("Connecting to url: {}", url);

    let response = client.put(url)
       .header("User-Agent", "actix-web/3.0")
       .send_json(
           &RequestBody{
               port
           }
       )
       .await
       .unwrap();  // Unwrap is fine here, there is no point in running a game server without a load balancer

    match response.status() {
        StatusCode::CREATED => println!("Successfully connected to a load balancer"),
        StatusCode::NO_CONTENT => println!("Reconnected to a load balancer"),
        // Panic is fine here, there is no point in running a game server without a load balancer
        _ => panic!("Invalid response from the load balancer: {:?}", response),
    }
}