use std::sync::RwLock;

use crate::server_id::ServerId;

pub struct GameServerRepo {
    servers: RwLock<Vec<String>>,
}

pub enum AddGameServerResult {
    CouldNotGetLock,
    ServerAlreadyRegistered(usize),
    ServerAdded(usize),
}

pub enum GetGameServerResult {
    CouldNotGetLock,
    Found(String),
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
            .position(|addr| addr == server_address);

        if let Some(position) = position {
            return AddGameServerResult::ServerAlreadyRegistered(position);
        }

        let server_id = servers.len();
        servers.push(server_address.to_string());

        println!("Added a new server! Address: {}, ID: {}", server_address, server_id);

        AddGameServerResult::ServerAdded(server_id)
    }

    pub fn get(&self, server_id: ServerId) -> GetGameServerResult {
        let servers = match self.servers.read() {
            Err(_) => return GetGameServerResult::CouldNotGetLock,
            Ok(repo) => repo,
        };

        match servers.get(server_id.into_inner()) {
            Some(server_address) => GetGameServerResult::Found(server_address.clone()),
            None => GetGameServerResult::NotFound,
        }
    }
}