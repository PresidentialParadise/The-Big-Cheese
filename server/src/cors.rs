use axum::body::{Body, BoxBody};
use axum::http::header::{ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_ORIGIN};
use axum::http::{HeaderValue, Method, Request, Response};
use futures::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};
use tower::{Layer, Service};

#[derive(Clone, Default)]
#[allow(clippy::module_name_repetitions)]
pub struct CorsLayer {}

impl<S> Layer<S> for CorsLayer {
    type Service = CorsService<S>;

    fn layer(&self, service: S) -> Self::Service {
        CorsService { service }
    }
}

#[derive(Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct CorsService<S> {
    service: S,
}

impl<S, Req, Resp> Service<Request<Req>> for CorsService<S>
where
    S: Service<Request<Req>, Response = Response<Resp>>,
    Resp: Empty,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = CorsFuture<S::Future, Resp, S::Error>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, request: Request<Req>) -> Self::Future {
        CorsFuture {
            cors_request: request.method() == Method::OPTIONS,
            inner: self.service.call(request),
            response: PhantomData::default(),
            error: PhantomData::default(),
        }
    }
}

#[pin_project::pin_project]
#[allow(clippy::module_name_repetitions)]
pub struct CorsFuture<Fut, Resp, Err> {
    cors_request: bool,
    #[pin]
    inner: Fut,
    response: PhantomData<Response<Resp>>,
    error: PhantomData<Err>,
}

impl<Fut, Resp: Empty, Err> Future for CorsFuture<Fut, Resp, Err>
where
    Fut: Future<Output = Result<Response<Resp>, Err>>,
{
    type Output = Result<Response<Resp>, Err>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.cors_request {
            let r = Response::new(Resp::new_empty());
            Poll::Ready(Ok(insert_headers(r)))
        } else {
            let this = self.project();
            match this.inner.poll(cx) {
                Poll::Ready(r) => Poll::Ready(r.map(insert_headers)),
                Poll::Pending => Poll::Pending,
            }
        }
    }
}

fn insert_headers<Body>(mut req: Response<Body>) -> Response<Body> {
    let headers = req.headers_mut();
    headers.insert(ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"));
    headers.insert(ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("*"));
    req
}

pub trait Empty {
    fn new_empty() -> Self;
}

impl Empty for Body {
    fn new_empty() -> Self {
        Body::empty()
    }
}

impl Empty for BoxBody {
    fn new_empty() -> Self {
        axum::body::box_body(Body::empty())
    }
}
