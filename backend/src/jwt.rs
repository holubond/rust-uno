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
    let claims = Claims::with_custom_claims(jwt_data, Duration::);
    key.authenticate(claims).unwrap()
}
