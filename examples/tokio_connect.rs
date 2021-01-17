use async_http_proxy::http_connect_tokio;
use std::error::Error;
use tokio::net::TcpStream;
// Features "runtime-tokio" have to be activated
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    http_connect_tokio(&mut stream, "example.org", 443).await?;
    // stream is now connect to github.com
    Ok(())
}
