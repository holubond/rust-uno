const ON_HEROKU: bool = true;

const METHOD: &str = "http";
const HOST: &str = "localhost";
const PORT: &str = "9000";
const WSMETHOD: &str = "ws";
const WSPORT: &str = "9000";

// Heroku
const HEROKU_METHOD: &str = "https";
const HEROKU_HOST: &str = "ancient-anchorage-67103.herokuapp.com";
const HEROKU_WSMETHOD: &str = "wss";

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
    if ON_HEROKU {
        return format!("{}://{}{}", HEROKU_METHOD, HEROKU_HOST, endpoint);
    }
    format!("{}://{}:{}{}", METHOD, HOST, PORT, endpoint)
}

pub fn game_ws(token: &String) -> String {
    let endpoint = format!("/ws/token/{}", token);
    if ON_HEROKU {
        return format!("{}://{}{}", HEROKU_WSMETHOD, HEROKU_HOST, endpoint);
    }
    format!("{}://{}:{}{}", WSMETHOD, HOST, WSPORT, endpoint)
}
