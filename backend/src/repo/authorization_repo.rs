use std::fmt::Error;
use actix::fut::result;
use actix_web::error::ParseError;
use actix_web::http::header::Header;
use actix_web::HttpRequest;
use actix_web_httpauth::headers::authorization::{Authorization, Bearer};
use jwt_simple::prelude::*;

pub struct AuthorizationRepo {
    pub key: HS256Key
}

#[derive(Serialize, Deserialize)]
pub struct JwtData {
    player_name: String,
    game_id: String,
}

impl AuthorizationRepo {
    pub fn new() -> AuthorizationRepo { Self { key: HS256Key::generate() } }

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

    pub fn verify_jwt(&self, player_name: String, game_id: String, claims: JWTClaims<JwtData>) -> bool {
        let is_author = claims.custom.game_id == game_id;
        let is_user = claims.custom.player_name == player_name;
        is_author && is_user
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

    pub fn user_from_token(&self, token: &String) -> Option<(String, String)>{
       return match self.key.verify_token::<JwtData>(&token, None) {
           Ok(claims) => Some((claims.custom.player_name, claims.custom.game_id)),
           Err(_) => None
       };
    }
}