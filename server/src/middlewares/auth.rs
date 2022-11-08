use actix_tls::accept::openssl::TlsStream;
use actix_web::{
    body::EitherBody,
    dev::{self, Extensions, Service, ServiceRequest, ServiceResponse, Transform},
    rt::net::TcpStream,
    Error, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use openssl::x509::X509;
use std::{
    any::Any,
    future::{ready, Ready},
};
use tracing::debug;

pub struct Auth;

impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service }))
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        let client_cert = request.conn_data::<X509>();
        let is_logged_in = client_cert.is_some();

        if !is_logged_in {
            let (request, _pl) = request.into_parts();

            let response = HttpResponse::Unauthorized().finish().map_into_right_body();

            return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
        }

        let res = self.service.call(request);

        Box::pin(async move { res.await.map(ServiceResponse::map_into_left_body) })
    }
}

pub fn get_client_cert(connection: &dyn Any, data: &mut Extensions) {
    if let Some(tls_socket) = connection.downcast_ref::<TlsStream<TcpStream>>() {
        debug!("TLS on_connect");

        let tls_session = tls_socket.ssl();

        if let Some(cert) = tls_session.peer_certificate() {
            debug!("Client certificate found");
            data.insert(cert);
        }
    } else {
        unreachable!("Socket should be TLS or plaintext");
    }
}
