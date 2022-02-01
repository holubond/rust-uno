use actix_web::HttpResponse;

use crate::err_msg::ErrMsg;

const SEPARATOR: &str = "@";

pub struct ServerId {
    id: usize,
}

impl ServerId {
    pub fn parse_full_id(full_game_id: String) -> Result<(Self, String), HttpResponse> {
        let split = full_game_id.split(SEPARATOR).collect::<Vec<&str>>();

        if split.len() != 2 {
            return Err(Self::invalid_game_id_response());
        }

        let server_id = match split.get(1) {
            None => return Err(Self::invalid_game_id_response()),
            Some(server_id) => server_id,
        };

        let game_id = match split.get(0) {
            None => return Err(Self::invalid_game_id_response()),
            Some(game_id) => game_id.to_string(),
        };

        match server_id.parse::<usize>() {
            Err(_) => Err(Self::invalid_game_id_response()),
            Ok(id) => Ok((Self { id }, game_id)),
        }
    }

    pub fn generate_full_id(game_id: String, server_id: usize) -> String {
        format!("{}{}{}", game_id, SEPARATOR, server_id)
    }

    pub fn into_inner(self) -> usize {
        self.id
    }

    fn invalid_game_id_response() -> HttpResponse {
        HttpResponse::BadRequest().json(
            ErrMsg::new("Invalid game ID".to_string())
        )
    }
}
