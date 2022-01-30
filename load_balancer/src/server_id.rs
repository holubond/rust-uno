use actix_web::HttpResponse;

const GAME_ID_PARTS: usize = 2;
const SEPARATOR: &str = "@";
const SERVER_ID_POSITION: usize = 1;

pub struct ServerId {
    id: usize,
}

impl ServerId {
    pub fn from(game_id: String) -> Result<Self, HttpResponse> {
        let split = game_id.split(SEPARATOR).collect::<Vec<&str>>();

        if split.len() != GAME_ID_PARTS {
            return Err(Self::invalid_game_id_response());
        }

        let server_id = match split.get(SERVER_ID_POSITION) {
            None => return Err(Self::invalid_game_id_response()),
            Some(server_id) => server_id,
        };

        match server_id.parse::<usize>() {
            Err(_) => Err(Self::invalid_game_id_response()),
            Ok(id) => Ok( Self{id} ),
        }
    }

    pub fn into_inner(&self) -> usize {
        self.id
    }

    fn invalid_game_id_response() -> HttpResponse {
        HttpResponse::BadRequest().body(
            "Invalid game ID"
        )
    }
}
