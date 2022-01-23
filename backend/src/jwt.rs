use jwt_simple::prelude::*;

#[derive(Serialize, Deserialize)]
struct JwtData {
    player_name: String,
    game_id: String,
}

pub(crate) fn generate_jwt(player_name: &String, game_id: &String) -> String {
    let jwt_data = JwtData {
        player_name: player_name.clone(),
        game_id: game_id.clone(),
    };
    let key = HS256Key::generate();
    let claims = Claims::with_custom_claims(jwt_data, Duration::from_hours(2));
    key.authenticate(claims).unwrap()
}

pub(crate) fn verify_jwt(player_name: String, game_id: String, token: String) -> bool {
    let key = HS256Key::generate();
    let token = rem_bearer(&token);
    let mut claims = key.verify_token::<JwtData>(&token, None);
    if claims.is_err() {
        return false
    }
    let claims = claims.unwrap();
    let is_author = claims.custom.game_id == game_id;
    let is_user = claims.custom.player_name == player_name;
    is_author && is_user
}

fn rem_bearer(value: &str) -> &str {
    let mut chars = value.chars();
    for _ in 0..6{
       chars.next();
    }
    chars.as_str()
}