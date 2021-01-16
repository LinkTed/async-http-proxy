use async_http_proxy::http_connect_async_std;
use async_std::net::TcpStream;
use async_std::task;
use std::error::Error;
use url::Url;
// Features "runtime-async-std" and "basic-auth" have to be activated
fn main() -> Result<(), Box<dyn Error>> {
    task::block_on(async {
        let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
        let url = Url::parse("https://github.com")?;

        http_connect_async_std(&mut stream, &url).await?;
        // stream is now connect to github.com
        Ok(())
    })
}
