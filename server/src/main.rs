use broker::server;
use tokio::net::TcpListener;

#[tokio::main]
pub async fn main() -> broker::Result<()> {


    // Bind a TCP listener
    let listener = TcpListener::bind(&format!("127.0.0.1:{}", "8080")).await?;

    server::run(listener).await;

    Ok(())
}
