use std::fmt::Error;
use actix::fut::result;
use actix_web::{error::ParseError, HttpResponse};
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

#[derive(Serialize, Debug)]
pub struct ErrResp<'a> {
    message: &'a str,
}

impl<'a> ErrResp<'a> {
    pub fn new(message: &'a str) -> Self {
        Self { message }
    }
}

type GameID = String;
type PlayerName = String;

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
            return Err( HttpResponse::Unauthorized().json( ErrResp::new("Invalid content of authorization header")) );
        }

        if parts[0] != "Bearer" {
            return Err( HttpResponse::Unauthorized().json( ErrResp::new("Missing 'Bearer' prefix in authorization header")) );
        }

        Ok(parts[1].into())
    }

    // Extracts and returns (game_id, player_name) from a request
    pub fn extract_data(&self, request: &HttpRequest) -> Result<(GameID, PlayerName), HttpResponse> {
        let auth_bearer = match Authorization::<Bearer>::parse(request) {
            Err(_) => return Err( HttpResponse::BadRequest().json(
                ErrResp::new("Request could not be parsed properly to extract authorisation header content"))
            ),
            Ok(x) => x.to_string(),
        };

        let token = self.remove_bearer_prefix(auth_bearer)?;

        match self.key.verify_token::<JwtData>(&token, None) {
            Err(_) => Err( HttpResponse::Unauthorized().json( ErrResp::new("Invalid JWT")) ),
            Ok(data) => Ok((data.custom.game_id, data.custom.player_name)),
        }    
    }
}