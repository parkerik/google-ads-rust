// cargo watch -c -w src -x run
pub mod google_ads;
pub mod http;

#[tokio::main]
async fn main() {
    http::serve().await;
}
