#![cfg_attr(docsrs, feature(doc_cfg))]
//! `async-http-proxy` is a lightweight asynchronous HTTP proxy client library, which can be used
//! to connect a to a TCP port via HTTP Connect proxy. It can use [Tokio](https://tokio.rs/) and
//! [async-std](https://async.rs/) as asynchronous runtime.  
//! # Example
//! The following example shows how to connect to `example.org` via Connect proxy (`tokio`):
//! ```ignore
//! use async_http_proxy::http_connect_tokio;
//! use std::error::Error;
//! use tokio::net::TcpStream;
//! // Features "runtime-tokio" have to be activated
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn Error>> {
//!     let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
//!     http_connect_tokio(&mut stream, "example.org", 443).await?;
//!     // stream is now connect to github.com
//!     Ok(())
//! }
//! ```
//!
//! The following example shows how to connect to `example.org` with Basic Authentication via
//! Connect proxy (`async-std`):
//! ```ignore
//! use async_http_proxy::http_connect_async_std_with_basic_auth;
//! use async_std::net::TcpStream;
//! use async_std::task;
//! use std::error::Error;
//! // Features "async-std-tokio" and "basic-auth" have to be activated
//! fn main() -> Result<(), Box<dyn Error>> {
//!     task::block_on(async {
//!         let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
//!         http_connect_async_std_with_basic_auth(
//!             &mut stream,
//!             "example.org",
//!             443,
//!             "username",
//!             "password",
//!         )
//!         .await?;
//!         // stream is now connect to github.com
//!         Ok(())
//!     })
//! }
//! ```

#[cfg(all(
    not(feature = "runtime-tokio"),
    not(feature = "runtime-async-std"),
    not(doc)
))]
compile_error!(
    "An async runtime have to be specified by feature: \"runtime-tokio\" \"runtime-async-std\""
);

mod request;
mod response;

#[cfg(feature = "runtime-async-std")]
use async_std::io::{Read, Write};
use httparse::Error as HttpParseError;
#[cfg(feature = "runtime-async-std")]
use response::recv_and_check_response_async_std;
#[cfg(feature = "runtime-tokio")]
use response::recv_and_check_response_tokio;
use std::io::Error as IoError;
use thiserror::Error as ThisError;
#[cfg(feature = "runtime-tokio")]
use tokio::io::{AsyncRead, AsyncWrite, BufStream};

/// The maximum length of the response header.
pub const MAXIMUM_RESPONSE_HEADER_LENGTH: usize = 4096;
/// The maximum HTTP Headers, which can be parsed.
pub const MAXIMUM_RESPONSE_HEADERS: usize = 16;

/// This enum contains all errors, which can occur during the HTTP `CONNECT`.
#[derive(Debug, ThisError)]
pub enum HttpError {
    #[error("IO Error: {0}")]
    IoError(#[from] IoError),
    #[error("HTTP parse error: {0}")]
    HttpParseError(#[from] HttpParseError),
    #[error("The maximum response header length is exceeded: {0}")]
    MaximumResponseHeaderLengthExceeded(String),
    #[error("The end of file is reached")]
    EndOfFile,
    #[error("No HTTP code was found in the response")]
    NoHttpCode,
    #[error("The HTTP code is not equal 200: {0}")]
    HttpCode200(u16),
    #[error("No HTTP reason was found in the response")]
    NoHttpReason,
    #[error("The HTTP reason is not equal 'ConnectionEstablished': {0}")]
    HttpReasonConnectionEstablished(String),
}

/// Connect to the server defined by the host and port and check if the connection was established.
///
/// The functions will use HTTP CONNECT request and the tokio runtime.
///
/// # Example
/// ```no_run
/// use async_http_proxy::http_connect_tokio;
/// use std::error::Error;
/// use tokio::net::TcpStream;
/// // Features "runtime-tokio" have to be activated
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn Error>> {
///     let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
///     http_connect_tokio(&mut stream, "example.org", 443).await?;
///     // stream is now connect to github.com
///     Ok(())
/// }
/// ```
#[cfg(feature = "runtime-tokio")]
#[cfg_attr(docsrs, doc(cfg(feature = "runtime-tokio")))]
pub async fn http_connect_tokio<IO>(io: &mut IO, host: &str, port: u16) -> Result<(), HttpError>
where
    IO: AsyncRead + AsyncWrite + Unpin,
{
    let mut stream = BufStream::new(io);

    request::send_request_tokio(&mut stream, host, port).await?;

    recv_and_check_response_tokio(&mut stream).await?;

    Ok(())
}

/// Connect to the server defined by the host and port with basic auth and check if the connection \
/// was established.
///
/// The functions will use HTTP CONNECT request and the tokio runtime.
///
/// # Example
/// use async_http_proxy::http_connect_tokio_with_basic_auth;
/// use std::error::Error;
/// use tokio::net::TcpStream;
/// // Features "runtime-tokio" and "basic-auth" have to be activated
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn Error>> {
///     let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
///     http_connect_tokio_with_basic_auth(&mut stream, "example.org", 443, "username", "password")
///         .await?;
///     // stream is now connect to github.com
///     Ok(())
/// }
/// ```no_run
/// ```
#[cfg(all(feature = "runtime-tokio", feature = "basic-auth"))]
#[cfg_attr(
    docsrs,
    doc(cfg(all(feature = "runtime-tokio", feature = "basic-auth")))
)]
pub async fn http_connect_tokio_with_basic_auth<IO>(
    io: &mut IO,
    host: &str,
    port: u16,
    username: &str,
    password: &str,
) -> Result<(), HttpError>
where
    IO: AsyncRead + AsyncWrite + Unpin,
{
    let mut stream = BufStream::new(io);

    request::send_request_tokio_with_basic_auth(&mut stream, host, port, username, password)
        .await?;

    recv_and_check_response_tokio(&mut stream).await?;

    Ok(())
}

/// Connect to the server defined by the host and port and check if the connection was established.
///
/// The functions will use HTTP CONNECT request and the tokio framework.
///
/// # Example
/// ```no_run
/// use async_http_proxy::http_connect_async_std;
/// use async_std::net::TcpStream;
/// use async_std::task;
/// use std::error::Error;
/// // Features "runtime-async-std" have to be activated
/// fn main() -> Result<(), Box<dyn Error>> {
///     task::block_on(async {
///         let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
///         http_connect_async_std(&mut stream, "example.org", 443).await?;
///         // stream is now connect to github.com
///         Ok(())
///     })
/// }
/// ```
#[cfg(feature = "runtime-async-std")]
#[cfg_attr(docsrs, doc(cfg(feature = "runtime-async-std")))]
pub async fn http_connect_async_std<IO>(io: &mut IO, host: &str, port: u16) -> Result<(), HttpError>
where
    IO: Read + Write + Unpin,
{
    request::send_request_async_std(io, host, port).await?;

    recv_and_check_response_async_std(io).await?;

    Ok(())
}

/// Connect to the server defined by the host and port with basic auth and check if the connection \
/// was established.
///
/// The functions will use HTTP CONNECT request and the async std framework.
///
/// # Example
/// ```no_run
/// use async_http_proxy::http_connect_async_std_with_basic_auth;
/// use async_std::net::TcpStream;
/// use async_std::task;
/// use std::error::Error;
/// // Features "async-std-tokio" and "basic-auth" have to be activated
/// fn main() -> Result<(), Box<dyn Error>> {
///     task::block_on(async {
///         let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
///         http_connect_async_std_with_basic_auth(
///             &mut stream,
///             "example.org",
///             443,
///             "username",
///             "password",
///         )
///         .await?;
///         // stream is now connect to github.com
///         Ok(())
///     })
/// }
/// ```
#[cfg(all(feature = "runtime-async-std", feature = "basic-auth"))]
#[cfg_attr(
    docsrs,
    doc(cfg(all(feature = "runtime-async-std", feature = "basic-auth")))
)]
pub async fn http_connect_async_std_with_basic_auth<IO>(
    io: &mut IO,
    host: &str,
    port: u16,
    username: &str,
    password: &str,
) -> Result<(), HttpError>
where
    IO: Read + Write + Unpin,
{
    request::send_request_async_std_with_basic_auth(io, host, port, username, password).await?;

    recv_and_check_response_async_std(io).await?;

    Ok(())
}
