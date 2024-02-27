use anyhow::Result;
use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub async fn read_msg<T: DeserializeOwned>(conn: &mut TcpStream) -> Result<T> {
    let buf = read_buf(conn).await?;
    if let Ok(msg) = rmp_serde::from_slice(&buf) {
        return Ok(msg);
    }
    let msg = serde_json::from_str(std::str::from_utf8(&buf)?)?;
    Ok(msg)
}
pub async fn read_string(conn: &mut TcpStream) -> Result<String> {
    Ok(String::from_utf8(read_buf(conn).await?)?)
}
pub async fn read_buf(conn: &mut TcpStream) -> Result<Vec<u8>> {
    let len = conn.read_u32().await?;
    let mut buf = vec![0; len as usize];
    conn.read_exact(&mut buf).await?;
    Ok(buf)
}

pub async fn write_msg<T: Serialize>(conn: &mut TcpStream, msg: &T) -> Result<()> {
    write_buf(conn, &rmp_serde::to_vec_named(msg)?).await
}
pub async fn write_string(conn: &mut TcpStream, s: &str) -> Result<()> {
    write_buf(conn, s.as_bytes()).await
}
pub async fn write_buf(conn: &mut TcpStream, buf: &[u8]) -> Result<()> {
    let len = buf.len() as u32;
    let mut b: Vec<u8> = len.to_be_bytes().to_vec();
    b.extend(buf);
    conn.write_all(&b).await?;
    conn.flush().await?;
    Ok(())
}
