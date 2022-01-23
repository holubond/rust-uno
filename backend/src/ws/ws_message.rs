use actix::Message;

/// WebSocket message that can be sent to a WebSocket connection
#[derive(Message)]
#[rtype(result = "()")]
pub struct WSMsg {
    pub msg: String,
}

// TODO - implement all types of WS messages
impl WSMsg {

    // This is a sample function, delete after implementation of others
    pub fn custom(msg: String) -> Self {
        Self { msg: msg }
    }

    pub fn status(/*...*/) -> Self {
        todo!("Implement me!");
    }
}
