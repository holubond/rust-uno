use jwt_simple::prelude::*;

pub struct AuthorizationRepo {
    pub key: HS256Key
}

#[derive(Serialize, Deserialize)]
struct JwtData {
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
        let claims = Claims::with_custom_claims(jwt_data, Duration::from_days(7));
        self.key.authenticate(claims).unwrap()
    }

    pub fn verify_jwt(&self, player_name: String, game_id: String, token: String) -> bool {
        let token = self.rem_bearer(&token);
        let claims = self.key.verify_token::<JwtData>(&token, None);
        if claims.is_err() {
            return false
        }
        let claims = claims.unwrap();
        let is_author = claims.custom.game_id == game_id;
        let is_user = claims.custom.player_name == player_name;
        is_author && is_user
    }

    pub fn rem_bearer(&self, value: &str) -> String {
        let mut chars = value.chars();
        for _ in 0..7 {
            chars.next();
        }
        chars.as_str().to_string()
    }
}