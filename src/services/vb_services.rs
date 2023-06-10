use actix::{Handler, StreamHandler};
use actix_web_actors::ws;

use crate::models::query_models::{VbConnect, VbDisconnect, VboardClient, VboardRes, VboardSrv};

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for VboardClient {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        if let Ok(ws::Message::Ping(msg)) = msg {
            ctx.pong(&msg)
        }
    }
}

impl Handler<VboardRes<'_>> for VboardClient {
    type Result = ();
    fn handle(&mut self, msg: VboardRes, ctx: &mut Self::Context) -> Self::Result {
        let res = msg.0;
        ctx.text(res.as_ref());
    }
}

impl Handler<VbConnect> for VboardSrv {
    type Result = ();
    fn handle(&mut self, msg: VbConnect, _ctx: &mut Self::Context) -> Self::Result {
        self.vb_addr.insert(msg.0);
        log::debug!(
            "New client connection.Total connection count : {}",
            self.vb_addr.len()
        );
    }
}
impl Handler<VbDisconnect> for VboardSrv {
    type Result = ();
    fn handle(&mut self, msg: VbDisconnect, _ctx: &mut Self::Context) -> Self::Result {
        self.vb_addr.remove(&msg.0);
        log::debug!(
            "Client Disconnected.Total connection count : {}",
            self.vb_addr.len()
        )
    }
}
impl Handler<VboardRes<'static>> for VboardSrv {
    type Result = ();
    fn handle(&mut self, msg: VboardRes<'static>, _ctx: &mut Self::Context) -> Self::Result {
        let vb_str = msg.0;
        self.vb_addr
            .iter()
            .for_each(|addr| addr.do_send(VboardRes(vb_str.clone())));
    }
}
