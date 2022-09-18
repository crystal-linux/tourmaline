#[tokio::main(flavor = "current_thread")]
async fn main() {
    color_eyre::install().unwrap();
    tourmaline::test_execute().await.unwrap();
}
