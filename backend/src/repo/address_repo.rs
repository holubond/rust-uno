use local_ip_address::local_ip;

pub struct AddressRepo {
    pub port: String,
}

impl AddressRepo {
    pub fn new(port: String) -> AddressRepo {
        Self { port }
    }

    pub fn full_local_address(&self) -> String {
        format!("{}:{}", local_ip().unwrap(), self.port)
    }
}
