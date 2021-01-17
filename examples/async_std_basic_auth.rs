use async_http_proxy::http_connect_async_std_with_basic_auth;
use async_std::net::TcpStream;
use async_std::task;
use std::error::Error;
// Features "async-std-tokio" and "basic-auth" have to be activated
fn main() -> Result<(), Box<dyn Error>> {
    task::block_on(async {
        let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
        http_connect_async_std_with_basic_auth(
            &mut stream,
            "example.org",
            443,
            "username",
            "password",
        )
        .await?;
        // stream is now connect to github.com
        Ok(())
    })
}
