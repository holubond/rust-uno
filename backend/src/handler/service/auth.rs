use actix_web::http::header::Header;
use actix_web::HttpRequest;
use actix_web::{error::ParseError, HttpResponse};
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
                    ErrMsg::new("Game id in the url does not match the one in JWT")
                )
            );
        }
        Ok(self.id)
    }
}

pub type PlayerName = String;

impl AuthService {
    pub fn new() -> AuthService {
        Self {
            key: HS256Key::generate(),
        }
    }

    pub fn generate_jwt(&self, player_name: &String, game_id: &String) -> String {
        let jwt_data = JwtData {
            player_name: player_name.clone(),
            game_id: game_id.clone(),
        };
        let claims = Claims::with_custom_claims(jwt_data, Duration::from_days(9000));
        self.key.authenticate(claims).unwrap()
    }

    pub fn valid_jwt(&self, token: &String) -> Result<JWTClaims<JwtData>, anyhow::Error> {
        let token = self.rem_bearer(&token);
        self.key.verify_token::<JwtData>(&token, None)
    }

    pub fn verify_jwt(
        &self,
        player_name: String,
        game_id: String,
        claims: JWTClaims<JwtData>,
    ) -> bool {
        let is_author = claims.custom.game_id == game_id;
        let is_user = claims.custom.player_name == player_name;
        is_author && is_user
    }

    pub fn user_from_claims(&self, claims: &JWTClaims<JwtData>) -> String {
        claims.custom.player_name.clone()
    }

    pub fn parse_jwt(&self, request: HttpRequest) -> Result<Authorization<Bearer>, ParseError> {
        Authorization::<Bearer>::parse(&request)
    }

    fn rem_bearer(&self, value: &str) -> String {
        let mut chars = value.chars();
        for _ in 0..7 {
            chars.next();
        }
        chars.as_str().to_string()
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

        match self.key.verify_token::<JwtData>(&token, None) {
            Err(_) => Err(HttpResponse::Unauthorized().json(ErrRespLocal::new("Invalid JWT"))),
            Ok(data) => Ok((
                GameID {
                    id: data.custom.game_id,
                },
                data.custom.player_name,
            )),
        }
    }

    pub fn extract_data_from_jwt(&self, jwt: String) -> Result<(String, PlayerName), HttpResponse> {
        match self.key.verify_token::<JwtData>(&jwt, None) {
            Err(_) => Err(HttpResponse::Unauthorized().json(ErrRespLocal::new("Invalid JWT"))),
            Ok(data) => Ok((data.custom.game_id, data.custom.player_name)),
        }
    }
}
