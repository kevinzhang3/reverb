use anyhow::Result;

mod router;
mod handlers;

#[tokio::main]
async fn main() -> Result<()> {
    let mut router = router::Router::new();

    router.method_get("/", handlers::base_uri);

    router.start("127.0.0.1:8080").await?;

    Ok(())
}
