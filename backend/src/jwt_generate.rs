use jwt_simple::prelude::*;

#[derive(Serialize, Deserialize)]
struct JwtData {
    player_name: String,
    game_id: String,
}

pub(crate) fn generate_jwt(player_name: String, game_id: String) -> String {
    let jwt_data = JwtData {
        player_name,
        game_id,
    };
    let key = HS256Key::generate();
    let claims = Claims::with_custom_claims(jwt_data, Duration::from_secs(30));
    key.authenticate(claims).unwrap()
}
