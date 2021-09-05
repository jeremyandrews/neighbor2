mod app;
mod server;

#[tokio::main]
async fn main() {
    server::run(app::DemoApp::default()).await;
}
