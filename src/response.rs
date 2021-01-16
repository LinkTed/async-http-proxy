use crate::{HttpError, MAXIMUM_RESPONSE_HEADERS, MAXIMUM_RESPONSE_HEADER_LENGTH};
use httparse::{Response, EMPTY_HEADER};

#[cfg(feature = "runtime-tokio")]
async fn get_response_tokio<IO>(stream: &mut tokio::io::BufStream<IO>) -> Result<String, HttpError>
where
    IO: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin,
{
    use tokio::io::AsyncBufReadExt;

    let mut response = String::new();
    loop {
        if stream.read_line(&mut response).await? == 0 {
            return Err(HttpError::EndOfFile);
        }

        if MAXIMUM_RESPONSE_HEADER_LENGTH < response.len() {
            return Err(HttpError::MaximumResponseHeaderLengthExceeded(response));
        }

        if response.ends_with("\r\n\r\n") {
            return Ok(response);
        }
    }
}

#[cfg(feature = "runtime-async-std")]
async fn get_response_async_std<R>(read: &mut R) -> Result<String, HttpError>
where
    R: async_std::io::Read + Unpin,
{
    use async_std::io::prelude::BufReadExt;

    let mut reader = async_std::io::BufReader::new(read);
    let mut response = String::new();
    loop {
        if reader.read_line(&mut response).await? == 0 {
            return Err(HttpError::EndOfFile);
        }

        if MAXIMUM_RESPONSE_HEADER_LENGTH < response.len() {
            return Err(HttpError::MaximumResponseHeaderLengthExceeded(response));
        }

        if response.ends_with("\r\n\r\n") {
            return Ok(response);
        }
    }
}

fn check_code(response: &Response<'_, '_>) -> Result<(), HttpError> {
    match response.code {
        Some(code) => {
            if code == 200 {
                Ok(())
            } else {
                Err(HttpError::HttpCode200(code))
            }
        }
        None => Err(HttpError::NoHttpCode),
    }
}

fn check_reason(response: &Response<'_, '_>) -> Result<(), HttpError> {
    match response.reason {
        Some(reason) => {
            if reason == "Connection Established" {
                Ok(())
            } else {
                Err(HttpError::HttpReasonConnectionEstablished(
                    reason.to_owned(),
                ))
            }
        }
        None => Err(HttpError::NoHttpReason),
    }
}

fn parse_and_check(response_string: &str) -> Result<(), HttpError> {
    let mut response_headers = [EMPTY_HEADER; MAXIMUM_RESPONSE_HEADERS];
    let mut response = Response::new(&mut response_headers[..]);
    response.parse(response_string.as_bytes())?;

    check_code(&response)?;
    check_reason(&response)?;

    Ok(())
}

#[cfg(feature = "runtime-tokio")]
pub(crate) async fn recv_and_check_response_tokio<IO>(
    stream: &mut tokio::io::BufStream<IO>,
) -> Result<(), HttpError>
where
    IO: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin,
{
    let response_string = get_response_tokio(stream).await?;

    parse_and_check(&response_string)?;

    Ok(())
}

#[cfg(feature = "runtime-async-std")]
pub(crate) async fn recv_and_check_response_async_std<R>(read: &mut R) -> Result<(), HttpError>
where
    R: async_std::io::Read + Unpin,
{
    let response_string = get_response_async_std(read).await?;

    parse_and_check(&response_string)?;

    Ok(())
}
