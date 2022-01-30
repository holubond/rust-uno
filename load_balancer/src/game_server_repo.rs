use std::sync::RwLock;

use crate::server_id::ServerId;

pub struct GameServerRepo {
    servers: RwLock<Vec<Server>>,
}

pub struct Server {
    games: usize,
    address: String,
}

pub enum AddGameServerResult {
    CouldNotGetLock,
    ServerAlreadyRegistered,
    ServerAdded,
}

pub enum GetGameServerError {
    CouldNotGetLock,
    NotFound,
}

pub enum GetServerForNewGameError {
    CouldNotGetLock,
    NoServerAvailable,
}

impl GameServerRepo {
    pub fn new() -> Self {
        Self {
            servers: RwLock::new(Vec::new()),
        }
    }

    /// Adds the game server address to a unique set of known servers.
    /// Returns ID of the server
    pub fn add(&self, server_address: &str) -> AddGameServerResult {
        let mut servers = match self.servers.write() {
            Err(_) => return AddGameServerResult::CouldNotGetLock,
            Ok(repo) => repo,
        };

        let position = servers
            .iter()
            .position(|server| server.address == server_address);

        if position.is_some() {
            return AddGameServerResult::ServerAlreadyRegistered;
        }

        let server_id = servers.len();

        let server = Server {
            address: server_address.to_string(),
            games: 0,
        };

        servers.push(server);

        println!(
            "Added a new server! Address: {}, ID: {}",
            server_address, server_id
        );

        AddGameServerResult::ServerAdded
    }

    pub fn get(&self, server_id: ServerId) -> Result<String, GetGameServerError> {
        let servers = match self.servers.read() {
            Err(_) => return Err(GetGameServerError::CouldNotGetLock),
            Ok(repo) => repo,
        };

        match servers.get(server_id.into_inner()) {
            None => Err(GetGameServerError::NotFound),
            Some(server) => Ok(server.address.clone()),
        }
    }

    pub fn get_server_for_new_game(&self) -> Result<(String, usize), GetServerForNewGameError> {
        let mut servers = match self.servers.write() {
            Err(_) => return Err(GetServerForNewGameError::CouldNotGetLock),
            Ok(repo) => repo,
        };

        let candidate = servers
            .iter_mut()
            .enumerate()
            .min_by(|(_, s1), (_, s2)| s1.games.cmp(&s2.games));

        let (server_id, server) = match candidate {
            None => return Err(GetServerForNewGameError::NoServerAvailable),
            Some(server) => server,
        };

        server.games += 1;

        Ok((server.address.clone(), server_id))
    }

    pub fn notify_about_false_game_create(&self, server_id: usize) {
        let mut servers =
            match self.servers.write() {
                Err(_) => return println!(
                    "game_server_repo.notify_about_false_game_create(): could not acquire a lock"
                ),
                Ok(repo) => repo,
            };

        let server = match servers.get_mut(server_id) {
            None => return println!(
                "game_server_repo.notify_about_false_game_create(): server with id {} not found",
                server_id
            ),
            Some(server) => server,
        };

        server.games -= 1;
    }
}
