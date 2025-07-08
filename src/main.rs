use anyhow::Result;

mod router;
mod handlers;

#[tokio::main]
async fn main() -> Result<()> {
    let mut router = router::Router::new();

    router.serve_static("/", "./public");
    router.method_get("/api", handlers::get_json);
    router.start("127.0.0.1:8080").await?;

    Ok(())
}
