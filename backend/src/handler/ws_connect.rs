use actix_web::{get, web, Error, HttpRequest, HttpResponse};

use crate::ws::{ws_conn::WSConn, ws_message::WSMsg};

#[get("/ws")]
pub async fn ws_connect(r: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    // Create a new WSConn
    let (conn, response) = WSConn::new(&r, stream)?;

    // The connection is able to receive WSMsg messages
    conn.send(WSMsg::custom("ABC".into())); // just an example, TODO remove me

    // TODO - Assign the WSConn to a player, create and send a STATUS message
    
    Ok(response)
}
