use std::future::{ready, Ready};

use actix_session::SessionExt;
use actix_web::body::{EitherBody, MessageBody};
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::HttpResponse;
use futures::future::LocalBoxFuture;

//Admin only guard middleware
// : )
pub struct AdminOnlyGuard;

impl<S, B> Transform<S, ServiceRequest> for AdminOnlyGuard
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;

    type Error = actix_web::Error;

    type InitError = ();
    type Transform = AdminOnlyGuardService<S>;

    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AdminOnlyGuardService { service }))
    }
}

pub struct AdminOnlyGuardService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AdminOnlyGuardService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;
    type Error = actix_web::Error;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let session = req.get_session();
        if let Some(true) = session.get::<bool>("admin").ok().flatten() {
            let fut = self.service.call(req);
            Box::pin(async move {
                let res = fut.await?;
                Ok(res.map_into_left_body())
            })
        } else {
            log::debug!("Unauthorized Access Request : [{}]", req.path());
            Box::pin(async move {
                Ok(ServiceResponse::<EitherBody<B>>::new(
                    req.into_parts().0,
                    HttpResponse::Unauthorized()
                        .body("Unauthorized Access request")
                        .map_into_right_body(),
                ))
            })
        }
    }
}
