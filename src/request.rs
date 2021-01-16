use crate::HttpError;
use url::Url;

fn get_port(url: &Url) -> Result<u16, HttpError> {
    match url.port_or_known_default() {
        Some(port) => Ok(port),
        None => Err(HttpError::NoPort),
    }
}

fn append_port(url: &Url, host: &mut String) -> Result<(), HttpError> {
    let port = get_port(url)?;
    let port_string = format!(":{}", port);
    host.push_str(&port_string);
    Ok(())
}

fn get_host(url: &Url) -> Result<String, HttpError> {
    match url.host_str() {
        Some(host) => Ok(host.to_owned()),
        None => Err(HttpError::NoHost),
    }
}

#[cfg(feature = "basic-auth")]
fn get_basic_auth(url: &Url) -> Option<String> {
    use base64::encode;
    let username = url.username();
    let password = url.password()?;
    let basic_auth = format!("{}:{}", username, password);
    let basic_auth = encode(basic_auth);
    Some(basic_auth)
}

#[cfg(feature = "basic-auth")]
fn append_basic_auth(request: &mut String, url: &Url) {
    if let Some(basic_auth) = get_basic_auth(url) {
        let proxy_authorization = format!("Proxy-Authorization: Basic {}\r\n", basic_auth);
        request.push_str(&proxy_authorization);
    }
}

fn make_request(url: &Url) -> Result<String, HttpError> {
    let mut host = get_host(url)?;
    append_port(url, &mut host)?;

    let mut request = format!(
        "CONNECT {0} HTTP/1.1\r\n\
         Host: {0}\r\n\
         Proxy-Connection: Keep-Alive\r\n",
        host,
    );

    #[cfg(feature = "basic-auth")]
    append_basic_auth(&mut request, url);

    request.push_str("\r\n");
    Ok(request)
}

#[cfg(feature = "runtime-tokio")]
pub(crate) async fn send_request_tokio<IO>(
    stream: &mut tokio::io::BufStream<IO>,
    url: &Url,
) -> Result<(), HttpError>
where
    IO: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin,
{
    use tokio::io::AsyncWriteExt;

    let request = make_request(url)?;

    stream.write_all(request.as_bytes()).await?;
    stream.flush().await?;

    Ok(())
}

#[cfg(feature = "runtime-async-std")]
pub(crate) async fn send_request_async_std<W>(write: &mut W, url: &Url) -> Result<(), HttpError>
where
    W: async_std::io::Write + Unpin,
{
    use async_std::io::prelude::WriteExt;

    let request = make_request(url)?;

    write.write_all(request.as_bytes()).await?;
    write.flush().await?;

    Ok(())
}
