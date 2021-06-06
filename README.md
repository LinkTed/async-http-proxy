# async-http-proxy
`async-http-proxy` is a lightweight asynchronous HTTP proxy client library, which can be used to 
connect a to a TCP port via HTTP Connect proxy. It can use [Tokio](https://tokio.rs/) and 
[async-std](https://async.rs/) as asynchronous runtime.  

[![Latest version](https://img.shields.io/crates/v/async-http-proxy.svg)](https://crates.io/crates/async-http-proxy)
[![License](https://img.shields.io/crates/l/async-http-proxy.svg)](https://opensource.org/licenses/BSD-3-Clause)
[![Dependency status](https://deps.rs/repo/github/linkted/async-http-proxy/status.svg)](https://deps.rs/repo/github/linkted/async-http-proxy)


## Example
The following example shows how to connect to `github.com` via Connect proxy (`tokio`):
```rust
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
```

The following example shows how to connect to `example.org` with Basic Authentication via Connect 
proxy (`async-std`):
```rust
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
```

## Features
- [x] HTTP `CONNECT`
- [x] Basic Auth
- [x] Tokio
- [x] async-std

## License
This project is licensed under the [BSD-3-Clause](https://opensource.org/licenses/BSD-3-Clause) 
license.

### Contribution
Any contribution intentionally submitted for inclusion in `async_http_proxy` by you, shall be 
licensed as [BSD-3-Clause](https://opensource.org/licenses/BSD-3-Clause), without any additional 
terms or conditions.
