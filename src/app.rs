use std::{
    future::Future,
    pin::Pin,
    sync::{atomic::AtomicUsize, Arc},
    task::Poll,
};
use axum::{
    body::Body,
    http::{Request, Response, HeaderValue},
};

pub struct DemoApp {
    counter: Arc<AtomicUsize>,
}

impl Default for DemoApp {
    fn default() -> Self {
        DemoApp {
            counter: Arc::new(AtomicUsize::new(0)),
        }
    }
}

impl tower::Service<Request<Body>> for DemoApp {
    type Response = Response<Body>;
    type Error = anyhow::Error;
    #[allow(clippy::type_complexity)]
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(())) // always ready to accept a connection
    }

    fn call(&mut self, mut req: Request<Body>) -> Self::Future {
        let counter = self.counter.clone();
        Box::pin(async move {
            println!("Handling a request: {}", req.uri());
            let counter = counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            anyhow::ensure!(counter % 4 != 2, "Failing 25% of the time...");
            req.headers_mut()
                .insert("X-Counter", HeaderValue::from_str(counter.to_string().as_ref()).unwrap());
            let res = Response::builder()
                .status(200)
                .body(Body::from(counter.to_string()))
                .expect("failed to create response");
            Ok::<_, anyhow::Error>(res)
        })
    }
}