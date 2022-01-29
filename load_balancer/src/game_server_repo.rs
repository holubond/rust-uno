use std::collections::HashMap;

pub struct GameServerRepo {
    servers: HashMap<String, String>,
}

impl GameServerRepo {
    pub fn new() -> Self {
        Self { servers: HashMap::new() }
    }
}