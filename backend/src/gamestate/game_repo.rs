use crate::gamestate::game::Game;
use actix_web::error::ErrorBadRequest;
use anyhow::anyhow;
use async_trait::async_trait;
use jwt_simple::prelude::*;
use nanoid::nanoid;
use std::fmt::Error;
use std::sync::Arc;

#[async_trait]
pub trait GameRepo {
    async fn create_game(&mut self, name: String) -> anyhow::Result<(Game)>;
}

#[derive(Clone)]
pub struct PostgresGameRepo {
    games: Vec<Game>,
}

impl PostgresGameRepo {
    pub fn new(games: Vec<Game>) -> Self {
        Self { games }
    }
}

#[async_trait]
impl GameRepo for PostgresGameRepo {
    async fn create_game(&mut self, name: String) -> anyhow::Result<(Game)> {
        if !name.is_empty() {
            let id = nanoid!(10);
            let key = HS256Key::generate();
            let claims = Claims::create(Duration::from_hours(2));
            let token = key.authenticate(claims)?;
            let mut game = Game::new(name);
            game.id = id;
            game.players
                .iter_mut()
                .find(|x| x.is_author)
                .map(|mut x| x.jwt = token);
            self.games.push(game.clone());

            return Ok(game);
        }
        Err(anyhow!("Empty name"))
    }
}
