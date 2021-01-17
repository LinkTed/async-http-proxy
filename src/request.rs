#[cfg(feature = "runtime-async-std")]
use async_std::io::{prelude::WriteExt, Write};
use std::io::Result;
#[cfg(feature = "runtime-tokio")]
use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt, BufStream};

#[cfg(feature = "basic-auth")]
fn get_proxy_authorization(username: &str, password: &str) -> String {
    let authorization = format!("{}:{}", username, password);
    let authorization = base64::encode(authorization.as_bytes());
    format!("Proxy-Authorization: Basic {}\r\n", authorization)
}

fn make_request(host: &str, port: u16) -> String {
    format!(
        "CONNECT {0}:{1} HTTP/1.1\r\n\
         Host: {0}:{1}\r\n\
         Proxy-Connection: Keep-Alive\r\n",
        host, port
    )
}

fn make_request_without_basic_auth(host: &str, port: u16) -> String {
    let mut request = make_request(host, port);
    request.push_str("\r\n");
    request
}

#[cfg(feature = "basic-auth")]
fn make_request_with_basic_auth(host: &str, port: u16, username: &str, password: &str) -> String {
    let mut request = make_request(host, port);
    let proxy_authorization = get_proxy_authorization(username, password);
    request.push_str(&proxy_authorization);
    request.push_str("\r\n");
    request
}

#[cfg(feature = "runtime-tokio")]
pub(crate) async fn send_request_tokio<IO>(
    stream: &mut BufStream<IO>,
    host: &str,
    port: u16,
) -> Result<()>
where
    IO: AsyncRead + AsyncWrite + Unpin,
{
    let request = make_request_without_basic_auth(host, port);

    stream.write_all(request.as_bytes()).await?;
    stream.flush().await
}

#[cfg(all(feature = "runtime-tokio", feature = "basic-auth"))]
pub(crate) async fn send_request_tokio_with_basic_auth<IO>(
    stream: &mut BufStream<IO>,
    host: &str,
    port: u16,
    username: &str,
    password: &str,
) -> Result<()>
where
    IO: AsyncRead + AsyncWrite + Unpin,
{
    let request = make_request_with_basic_auth(host, port, username, password);

    stream.write_all(request.as_bytes()).await?;
    stream.flush().await
}

#[cfg(feature = "runtime-async-std")]
pub(crate) async fn send_request_async_std<W>(write: &mut W, host: &str, port: u16) -> Result<()>
where
    W: Write + Unpin,
{
    let request = make_request_without_basic_auth(host, port);

    write.write_all(request.as_bytes()).await?;
    write.flush().await
}

#[cfg(all(feature = "runtime-async-std", feature = "basic-auth"))]
pub(crate) async fn send_request_async_std_with_basic_auth<W>(
    write: &mut W,
    host: &str,
    port: u16,
    username: &str,
    password: &str,
) -> Result<()>
where
    W: Write + Unpin,
{
    let request = make_request_with_basic_auth(host, port, username, password);

    write.write_all(request.as_bytes()).await?;
    write.flush().await
}
