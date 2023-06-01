use actix::{Handler, StreamHandler};
use actix_web_actors::ws;

use crate::models::query_models::{Vboard, VboardRes};

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Vboard {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        if let Ok(ws::Message::Ping(msg)) = msg {
            ctx.pong(&msg)
        }
    }
}

impl Handler<VboardRes<'_>> for Vboard {
    type Result = ();
    fn handle(&mut self, msg: VboardRes, ctx: &mut Self::Context) -> Self::Result {
        let res = msg.0;
        ctx.text(res.as_ref());
    }
}
