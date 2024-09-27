#[tokio::main(flavor = "current_thread")]
async fn main() {
    mqjs::realmain(std::env::args()).await;
}
