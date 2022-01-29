use actix_web::http::header::Header;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web_httpauth::headers::authorization::{Authorization, Bearer};
use jwt_simple::prelude::*;
use crate::handler::util::response::ErrMsg;

pub struct AuthService {
    pub key: HS256Key,
}

#[derive(Serialize, Deserialize)]
pub struct JwtData {
    player_name: String,
    game_id: String,
}

#[derive(Serialize, Debug)]
pub struct ErrRespLocal<'a> {
    message: &'a str,
}

impl<'a> ErrRespLocal<'a> {
    pub fn new(message: &'a str) -> Self {
        Self { message }
    }
}

pub struct GameID {
    id: String,
}

impl GameID {
    pub fn check(self, game_id: String) -> Result<String, HttpResponse> {
        if self.id != game_id {
            return Err(
                HttpResponse::Forbidden().json(
                    ErrMsg::new_from_scratch("Game id in the url does not match the one in JWT")
                )
            );
        }
        Ok(self.id)
    }

    pub fn into_inner(self) -> String {
        self.id
    }
}

pub struct PlayerName {
    name: String,
}

impl PlayerName {
    pub fn check(&self, author: &str) -> Result<(), HttpResponse> {
        if self.name != author {
            return Err(
                HttpResponse::Forbidden().json(
                    ErrMsg::new_from_scratch("This action can be done only by the author of the game")
                )
            );
        }
        Ok(())
    }

    pub fn into_inner(self) -> String {
        self.name
    }
}

impl AuthService {
    pub fn new() -> AuthService {
        Self {
            key: HS256Key::generate(),
        }
    }

    pub fn generate_jwt(&self, player_name: &str, game_id: &str) -> String {
        let jwt_data = JwtData {
            player_name: player_name.to_string(),
            game_id: game_id.to_string(),
        };
        let claims = Claims::with_custom_claims(jwt_data, Duration::from_days(9000));
        self.key.authenticate(claims).unwrap()
    }

    // Extracts and returns (game_id, player_name) from a request
    pub(in crate::handler) fn extract_data(
        &self,
        request: &HttpRequest,
    ) -> Result<(GameID, PlayerName), HttpResponse> {
        let auth_bearer =
            match Authorization::<Bearer>::parse(request) {
                Err(_) => return Err(HttpResponse::BadRequest().json(ErrRespLocal::new(
                    "Request could not be parsed properly to extract authorisation header content",
                ))),
                Ok(x) => x.to_string(),
            };

        let token = self.remove_bearer_prefix(auth_bearer)?;

        self.extract_data_from_token(token)
    }

    pub fn extract_data_from_token(&self, token: String) -> Result<(GameID, PlayerName), HttpResponse> {
        match self.key.verify_token::<JwtData>(&token, None) {
            Err(_) => Err(HttpResponse::Unauthorized().json(ErrRespLocal::new("Invalid JWT"))),
            Ok(data) => Ok((
                GameID {
                    id: data.custom.game_id,
                },
                PlayerName {
                    name: data.custom.player_name,
                }
            )),
        }
    }

    fn remove_bearer_prefix(&self, auth_bearer: String) -> Result<String, HttpResponse> {
        let parts = auth_bearer.split_whitespace().collect::<Vec<&str>>();

        if parts.len() != 2 {
            return Err(HttpResponse::Unauthorized()
                .json(ErrRespLocal::new("Invalid content of authorization header")));
        }

        if parts[0] != "Bearer" {
            return Err(HttpResponse::Unauthorized().json(ErrRespLocal::new(
                "Missing 'Bearer' prefix in authorization header",
            )));
        }

        Ok(parts[1].into())
    }
}
