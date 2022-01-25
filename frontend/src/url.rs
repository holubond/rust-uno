const METHOD: &str = "http";
const HOST: &str = "http://localhost";
const PORT: &str = "9000";

pub fn game() -> String {
    route("/game".into())
}

pub fn player(game_id: String) -> String {
    route(format!("/game/{}/player", game_id))
}

pub fn status_running(game_id: String) -> String {
    route(format!("/game/{}/statusRunning", game_id))
}

fn route(endpoint: String) -> String {
    format!("{}://{}:{}{}", METHOD, HOST, PORT, endpoint)
}
