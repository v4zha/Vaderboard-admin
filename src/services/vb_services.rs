use actix::{AsyncContext, ContextFutureSpawner, Handler, StreamHandler, WrapFuture};
use actix_web_actors::ws;

use crate::models::query_models::{
    TransferType, VbConnect, VbDisconnect, VboardClient, VboardGet, VboardRes, VboardSrv,
};

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for VboardClient {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        use ws::Message::*;
        match msg {
            Ok(Ping(msg)) => ctx.pong(&msg),
            Ok(Text(_)) => self
                .srv_addr
                .do_send(VboardGet(TransferType::Unicast(self.addr.clone().unwrap()))),
            _ => {}
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
    fn handle(&mut self, msg: VbConnect, ctx: &mut Self::Context) -> Self::Result {
        let addr = ctx.address();
        self.vb_addr.insert(msg.0.clone());
        log::debug!(
            "New client connection.Total connection count : {}",
            self.vb_addr.len()
        );
        addr.do_send(VboardGet(TransferType::Unicast(msg.0)));
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
        if let Some(transfer) = msg.1 {
            match transfer {
                TransferType::Unicast(addr) => addr.do_send(VboardRes(vb_str, None)),
                TransferType::Broadcast => {
                    self.vb_addr
                        .iter()
                        .for_each(|addr| addr.do_send(VboardRes(vb_str.clone(), None)));
                }
            }
        }
    }
}
impl Handler<VboardGet> for VboardSrv {
    type Result = ();
    fn handle(&mut self, msg: VboardGet, ctx: &mut Self::Context) -> Self::Result {
        let addr = ctx.address();
        let event_lock = self.app_state.clone();
        let db_pool = self.db_pool.clone();
        let vb_count = self.app_state.vb_count;
        async move {
            let event = event_lock.as_ref().current_event.lock().await;
            if let Some(e) = event.as_ref() {
                let vb_res = e.get_vboard(&db_pool, vb_count).await;
                match vb_res {
                    Ok(vb_str) => addr.do_send(VboardRes(vb_str, Some(msg.0))),
                    Err(e) => log::debug!("Error sending Vaderboard : {}", e),
                }
            }
        }
        .into_actor(self)
        .wait(ctx)
    }
}
