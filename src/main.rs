use anyhow::Result;
use reverb::routing::Router;


#[tokio::main]
async fn main() -> Result<()> {
    let mut router = Router::new();

    router.serve_static("/", "./public");

    router.debug(true);
    router.start("127.0.0.1:8080").await?;

    Ok(())
}
