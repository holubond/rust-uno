use std::sync::RwLock;

use crate::server_id::ServerId;

pub struct GameServerRepo {
    servers: RwLock<Vec<Server>>,
}

struct Server {
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

impl GameServerRepo {
    pub fn new() -> Self {
        Self { servers: RwLock::new(Vec::new()) }
    }

    /// Adds the game server address to a unique set of known servers.
    /// Returns ID of the server
    pub fn add(&self, server_address: &str) -> AddGameServerResult {

        let mut servers = match self.servers.write() {
            Err(_) => return AddGameServerResult::CouldNotGetLock,
            Ok(repo) => repo,
        };

        let position = servers.iter()
            .position(|server| server.address == server_address);

        if position.is_some() {
            return AddGameServerResult::ServerAlreadyRegistered;
        }

        let server_id = servers.len();

        let server = Server{
            address: server_address.to_string(),
            games: 0
        };

        servers.push(server);

        println!("Added a new server! Address: {}, ID: {}", server_address, server_id);

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
}