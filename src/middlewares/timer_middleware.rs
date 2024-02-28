use std::future::{ready, Ready};
use std::time::Duration;

use actix_web::http::header;
use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::LocalBoxFuture;

pub struct TimerMiddleware;

impl<S, B> Transform<S, ServiceRequest> for TimerMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = TimerMiddlewareTransform<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(TimerMiddlewareTransform { service }))
    }
}

pub struct TimerMiddlewareTransform<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for TimerMiddlewareTransform<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let start = std::time::Instant::now();

        let fut = self.service.call(req);

        Box::pin(async move {
            let mut res = fut.await?;

            let elapsed = start.elapsed();

            log::info!("Execution time: {}ms", elapsed.as_millis());
            log::info!(
                "{} {} {}ms",
                res.request().method(),
                res.request().uri(),
                elapsed.as_millis()
            );

            res.headers_mut().insert(
                HeaderName::from_static("X-Response-Time"),
                HeaderValue::from_str(&format!("{}ms", elapsed.as_millis())).unwrap(),
            );
            Ok(res)
        })
    }
}
