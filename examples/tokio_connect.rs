use async_http_proxy::http_connect_tokio;
use std::error::Error;
use tokio::net::TcpStream;
use url::Url;
// Features "runtime-tokio" and "basic-auth" have to be activated
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    let url = Url::parse("https://github.com")?;

    http_connect_tokio(&mut stream, &url).await?;
    // stream is now connect to github.com
    Ok(())
}
