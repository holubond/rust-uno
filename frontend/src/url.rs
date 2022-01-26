const METHOD: &str = "http";
const HOST: &str = "localhost";
const PORT: &str = "9000";
const WSMETHOD: &str = "ws";
const WSPORT: &str = "9000";

pub fn game() -> String {
    route("/game".into())
}

pub fn player(game_id: String) -> String {
    route(format!("/game/{}/player", game_id))
}

pub fn status_running(game_id: String) -> String {
    route(format!("/game/{}/statusRunning", game_id))
}

pub fn drawn_cards(game_id: String) -> String {
    route(format!("/game/{}/drawnCards", game_id))
}

pub fn play_card(game_id: String) -> String {
    route(format!("/game/{}/playCard", game_id))
}

fn route(endpoint: String) -> String {
    format!("{}://{}:{}{}", METHOD, HOST, PORT, endpoint)
}

pub fn game_ws(token: &String) -> String {
    let endpoint = format!("/ws/token/{}", token);
    format!("{}://{}:{}{}", WSMETHOD, HOST, WSPORT, endpoint)
}
