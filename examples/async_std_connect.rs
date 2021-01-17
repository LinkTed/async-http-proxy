use async_http_proxy::http_connect_async_std;
use async_std::net::TcpStream;
use async_std::task;
use std::error::Error;
// Features "runtime-async-std" have to be activated
fn main() -> Result<(), Box<dyn Error>> {
    task::block_on(async {
        let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
        http_connect_async_std(&mut stream, "example.org", 443).await?;
        // stream is now connect to github.com
        Ok(())
    })
}
