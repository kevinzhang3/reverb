use anyhow::Result;

mod router;
mod handlers;

#[tokio::main]
async fn main() -> Result<()> {
    let mut router = router::Router::new();

    router.GET("/", handlers::base_uri);

    Ok(())
}
