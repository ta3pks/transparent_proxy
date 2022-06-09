use std::error::Error;

use tap::Tap;
use tokio::{
	io::{AsyncReadExt, AsyncWriteExt},
	net::TcpStream,
};

use crate::Flags;

pub async fn handle_client(
	mut client: tokio::net::TcpStream,
	flags: Flags,
) -> Result<(), Box<dyn Error>>
{
	let mut server = TcpStream::connect(flags.target_addr).await?;
	if flags.password.is_some() || flags.username.is_some() {
		server.write_all(&[0x05, 0x02, 0x00, 0x02]).await?;
	} else {
		//no auth
		server.write_all(&[0x05, 0x01, 0x00]).await?;
	}
	let mut buf = [0u8; 2];
	server.read_exact(&mut buf).await?;
	match buf[1] {
		0xFF => return Err("auth failed".into()),
		0x00 => (), //nothing to do no password auth
		0x02 => {
			//password auth
			handle_password_auth(
				&mut server,
				flags.username.unwrap_or_default().as_str(),
				flags.password.unwrap_or_default().as_str(),
			)
			.await?;
		}
		_ => return Err("unsupported auth method".into()),
	}
	disregard_negotiations_of_client(&mut client).await?;
	let (mut client_read, mut client_write) = client.into_split();
	let (mut server_read, mut server_write) = server.into_split();
	tokio::spawn(async move {
		tokio::io::copy(&mut client_read, &mut server_write)
			.await
			.ok();
		dbg!("client_read finished");
	});
	tokio::spawn(async move {
		tokio::io::copy(&mut server_read, &mut client_write)
			.await
			.ok();
		dbg!("server_read finished");
	});

	Ok(())
}
async fn disregard_negotiations_of_client(client: &mut TcpStream) -> Result<(), Box<dyn Error>>
{
	let mut buf = [0u8; 2];
	client.read_exact(&mut buf).await?;
	client.read_exact(&mut vec![0u8; buf[1] as usize]).await?; //disregard method negotiation since we have done it already
	client.write_all(&[0x05, 0x00]).await?; // respond to client no password
	Ok(())
}
async fn handle_password_auth(
	sock: &mut TcpStream,
	username: &str,
	password: &str,
) -> Result<(), Box<dyn Error>>
{
	//{{{
	sock.write(&[0x01, username.len() as u8]).await?;
	sock.write(username.as_bytes()).await?;
	sock.write(&[password.len() as u8]).await?;
	sock.write(password.as_bytes()).await?;
	sock.flush().await?;
	let mut buf = [0u8; 2];
	sock.read_exact(&mut buf).await.tap_dbg(|_| {
		dbg!("auth resp", buf);
	})?;
	if buf[1] != 0x00 {
		return Err("auth failed".into());
	}
	Ok(())
} //}}}
