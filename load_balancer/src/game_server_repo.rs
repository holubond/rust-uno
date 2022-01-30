use std::sync::RwLock;

pub struct GameServerRepo {
    servers: RwLock<Vec<String>>,
}

pub enum AddGameServerResult {
    CouldNotGetLock,
    ServerAlreadyRegistered(usize),
    ServerAdded(usize),
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

        let position = servers.len();
        servers.push(server_address.to_string());

        AddGameServerResult::ServerAdded(position)
    }
}