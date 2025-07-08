use anyhow::Result;

mod router;


#[tokio::main]
async fn main() -> Result<()> {
    let mut router = router::Router::new();
    
    router.add()

    Ok(())
}
