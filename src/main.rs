use anyhow::Result;

mod router;
mod handlers;

#[tokio::main]
async fn main() -> Result<()> {
    let mut router = router::Router::new();

    router.serve_static("/", "./public");

    router.start("127.0.0.1:8080").await?;

    Ok(())
}
