use std::{error::Error, io::ErrorKind};

use tap::Tap;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

use crate::Flags;

pub async fn handle_client(
    mut client: tokio::net::TcpStream,
    flags: Flags,
) -> Result<(), Box<dyn Error>> {
    let auth_line = format!(
        "Proxy-Authorization: Basic {}\r\n",
        base64::encode(&format!(
            "{}:{}",
            flags
                .username
                .unwrap_or_default()
                .replace("%R", fastrand::u32(0..u32::MAX).to_string().as_str())
                .tap_dbg(|s| println!("username: {}", s)),
            flags.password.unwrap_or_default()
        ))
    );
    let mut server = TcpStream::connect(flags.target_addr).await?;
    let (mut reader, mut writer) = client.split();
    let buf_reader = BufReader::new(&mut reader);
    let mut lines = buf_reader.lines();
    let first_line = lines
        .next_line()
        .await?
        .ok_or_else(|| std::io::Error::new(ErrorKind::InvalidData, "missing init data"))?;
    dbg!(&first_line);
    server
        .write_all(format!("{first_line}\r\n{auth_line}\r\n").as_bytes())
        .await?;
    let (mut server_reader, mut server_writer) = server.split();
    let res = tokio::try_join!(
        tokio::io::copy(&mut reader, &mut server_writer),
        tokio::io::copy(&mut server_reader, &mut writer)
    );
    if cfg!(debug_assertions) {
        eprintln!("{res:?}");
    }
    Ok(())
}
