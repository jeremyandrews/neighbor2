use tower::{Service, ServiceExt};

use axum::{
    body::Body,
    http::{Request, Response},
};

pub async fn run<App>(mut app: App)
where
    App: Service<Request<Body>, Response = Response<Body>>,
    App::Error: std::fmt::Debug,
    App::Future: Send + 'static,
{
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        let req = Request::builder()
            .method("GET")
            .body(Body::from("foo"))
            .expect("failed to create request");

        let app = match app.ready().await {
            Err(e) => {
                eprintln!("Service not able to accept requests: {:?}", e);
                continue;
            }
            Ok(app) => app,
        };
        let future = app.call(req);
        tokio::spawn(async move {
            match future.await {
                Ok(res) => println!("Successful response: {:?}", res),
                Err(e) => eprintln!("Error occurred: {:?}", e),
            }
        });
    }
}