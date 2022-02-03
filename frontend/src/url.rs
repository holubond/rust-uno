const ON_HEROKU: bool = true;

const METHOD: &str = "http";
const HOST: &str = "localhost";
const LBPORT: &str = "9900";
const WSMETHOD: &str = "ws";

// Heroku
const HEROKU_METHOD: &str = "https";
const HEROKU_HOST: &str = "rust-uno.herokuapp.com";
const HEROKU_WSMETHOD: &str = "wss";

pub fn game() -> String {
    route_lb("/game".into())
}

pub fn player(game_id: String) -> String {
    route_lb(format!("/gameServer/{}", game_id))
}

pub fn player_gs(game_id: String, game_server: String) -> String {
    route_gs(format!("/game/{}/player", game_id), game_server)
}

pub fn status_running(game_id: String, game_server: String) -> String {
    route_gs(format!("/game/{}/statusRunning", game_id), game_server)
}

pub fn drawn_cards(game_id: String, game_server: String) -> String {
    route_gs(format!("/game/{}/drawnCards", game_id), game_server)
}

pub fn play_card(game_id: String, game_server: String) -> String {
    route_gs(format!("/game/{}/playCard", game_id), game_server)
}

fn route_lb(endpoint: String) -> String {
    if ON_HEROKU {
        return format!("{}://{}{}", HEROKU_METHOD, HEROKU_HOST, endpoint);
    }
    format!("{}://{}:{}{}", METHOD, HOST, LBPORT, endpoint)
}
fn route_gs(endpoint: String, game_server: String) -> String {
    if ON_HEROKU {
        return format!("{}://{}{}", HEROKU_METHOD, game_server, endpoint);
    }
    format!("{}://{}{}", METHOD, game_server, endpoint)
}

pub fn game_ws(token: &str, game_server: String) -> String {
    let endpoint = format!("/ws/token/{}", token);
    if ON_HEROKU {
        return format!("{}://{}{}", HEROKU_WSMETHOD, game_server, endpoint);
    }
    format!("{}://{}{}", WSMETHOD, game_server, endpoint)
}
