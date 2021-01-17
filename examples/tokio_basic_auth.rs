use async_http_proxy::http_connect_tokio_with_basic_auth;
use std::error::Error;
use tokio::net::TcpStream;
// Features "runtime-tokio" and "basic-auth" have to be activated
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    http_connect_tokio_with_basic_auth(&mut stream, "example.org", 443, "username", "password")
        .await?;
    // stream is now connect to github.com
    Ok(())
}
