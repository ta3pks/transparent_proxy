use std::{
	error::Error, net::{IpAddr, SocketAddr, ToSocketAddrs}, process::exit
};

use structopt::StructOpt;

use tokio::{
	io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}
};
fn resolve_sock_addr(addr: &str) -> Result<SocketAddr, String>
{
	let mut hosts = addr.to_socket_addrs().map_err(|e| e.to_string())?;
	let host = if let Some(host) = hosts.clone().find(|h| h.is_ipv4()) {
		Some(host)
	} else {
		hosts.next()
	}
	.ok_or_else(|| "no valid target host".to_string())?;

	Ok(host)
}

#[derive(StructOpt, Debug, Clone)]
struct Flags
{
	#[structopt(short, long, default_value = "127.0.0.1")]
	host: IpAddr,
	#[structopt(short = "P", long, default_value = "55000")]
	port: u16,
	#[structopt(short, long)]
	username: Option<String>,
	#[structopt(short, long)]
	password: Option<String>,
	#[structopt(short, long,parse(try_from_str = resolve_sock_addr))]
	target_addr: SocketAddr,
}
#[tokio::main]
async fn main()
{
	let flags = Flags::from_args();
	if (flags.username.is_none() && flags.password.is_some())
		|| (flags.username.is_some() && flags.password.is_none())
	{
		eprintln!("username is required when password is provided or vice versa");
		exit(1);
	}
	println!("{:?}", flags);
	let listener = TcpListener::bind((flags.host, flags.port)).await.unwrap();
	while let Ok((sock, _)) = listener.accept().await {
		let flags = flags.clone();
		tokio::spawn(async move {
			if let Err(e) = handle_client(sock, flags).await {
				eprintln!("{}", e);
			}
		});
	}
}
async fn handle_client(mut sock: tokio::net::TcpStream, flags: Flags)
	-> Result<(), Box<dyn Error>>
{
	let mut conn = TcpStream::connect(flags.target_addr).await?;
	if flags.password.is_some() || flags.username.is_some() {
		conn.write_all(&[0x05, 0x02, 0x00, 0x02]).await?;
	} else {
		//no auth
		conn.write_all(&[0x05, 0x01, 0x00]).await?;
	}
	let mut buf = [0u8; 2];
	sock.read_exact(&mut buf).await?;
	match buf[1] {
		0xFF => return Err("auth failed".into()),
		0x00 => (), //nothing to do no password auth
		0x02 => {
			//password auth
			handle_password_auth(
				&mut sock,
				flags.username.unwrap_or_default().as_str(),
				flags.password.unwrap_or_default().as_str(),
			)
			.await?;
		}
		_ => return Err("unsupported auth method".into()),
	}

	Ok(())
}
async fn handle_password_auth(
	sock: &mut TcpStream,
	username: &str,
	password: &str,
) -> Result<(), Box<dyn Error>>
{
	sock.write(&[0x01, username.len() as u8]).await?;
	sock.write(username.as_bytes()).await?;
	sock.write(&[password.len() as u8]).await?;
	sock.write(password.as_bytes()).await?;
	sock.flush().await?;
	let mut buf = [0u8; 2];
	sock.read_exact(&mut buf).await?;
	if buf[1] != 0x00 {
		return Err("auth failed".into());
	}
	Ok(())
}
