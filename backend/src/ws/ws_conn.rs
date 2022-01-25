use actix::Addr;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

use super::{ws_actor::WSActor, ws_message::WSMsg};

/// WebSocket connection to which it is possible to send messages
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct WSConn {
    addr: Addr<WSActor>,
}

impl WSConn {
    /// Create a new WebSocket connection from a request
    pub fn new(request: &HttpRequest, stream: web::Payload) -> Result<(Self, HttpResponse), Error> {
        let (addr, response) = ws::start_with_addr(WSActor::new(), request, stream)?;
        Ok((Self { addr }, response))
    }

    /// Send a WebSocket message
    pub fn send(&self, msg: WSMsg) {
        self.addr.do_send(msg);
    }
}
