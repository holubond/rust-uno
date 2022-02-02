use actix_web::{client::Client, http::StatusCode};
use serde::Serialize;

pub struct LoadBalancerConnector {
    lb_addr: String,
    gs_addr: String,
}

#[derive(Serialize, Debug)]
struct RequestBody {
    server: String,
}

impl LoadBalancerConnector {
    pub fn new(lb_addr: String, gs_addr: String) -> Self {
        Self { lb_addr, gs_addr }
    }

    pub async fn connect(&self) -> Result<(), ()> {
        let url = format!("http://{}/gameServer", self.lb_addr);
        println!("Connecting to url: {}", url);

        let client = Client::default();
        let response = client
            .put(url)
            .header("User-Agent", "actix-web/3.0")
            .send_json(&RequestBody {
                server: self.gs_addr.clone(),
            })
            .await;

        let response = match response {
            Err(err) => {
                println!("Failed with error: {}", err);
                return Err(());
            }
            Ok(x) => x,
        };

        match response.status() {
            StatusCode::CREATED => println!("Successfully connected to a load balancer"),
            StatusCode::NO_CONTENT => println!("Reconnected to a load balancer"),
            _ => {
                println!("Invalid response from the load balancer: {:?}", response);
                return Err(());
            }
        }

        Ok(())
    }
}
