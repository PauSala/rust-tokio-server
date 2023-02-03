use broker::client;
use tokio::time;

#[tokio::main]
pub async fn main() -> broker::Result<()> {
    let client = client::connect("localhost:8080").await;
    let mut client = client.unwrap();
    let mut counter = 1;
    let mut interval = time::interval(time::Duration::from_secs(1));
    loop  {
        interval.tick().await;
        let msg = format!("Message {} from client", counter);
        client.write_something(&msg).await?;
        client.read_response().await?;
        counter += 1;
        if counter > i32::MAX {
            counter = 0;
        }
    }

}
