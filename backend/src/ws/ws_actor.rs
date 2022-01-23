use std::time::{Duration, Instant};

use actix::{Actor, ActorContext, AsyncContext, Handler, StreamHandler};
use actix_web_actors::ws;

use super::ws_message::WSMsg;

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// Actor representing a WebSocket connection
pub struct WSActor {
    /// Heartbeat to keep the connection alive
    hb: Instant,
}

impl WSActor {
    pub fn new() -> Self {
        Self { hb: Instant::now() }
    }

    /// sends ping to client every HEARTBEAT_INTERVAL, checks heartbeats from client
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }
}

impl Handler<WSMsg> for WSActor {
    type Result = ();

    fn handle(&mut self, msg: WSMsg, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.msg)
    }
}

impl Actor for WSActor {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }
}

/// Handler for messages coming from the client
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WSActor {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(_)) => (),
            Ok(ws::Message::Binary(_)) => (),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}
