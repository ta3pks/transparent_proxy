use std::net::{IpAddr, SocketAddr, ToSocketAddrs};

use structopt::StructOpt;

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
pub struct Flags
{
	#[structopt(short, long, default_value = "127.0.0.1")]
	pub host: IpAddr,
	#[structopt(short = "P", long, default_value = "55000")]
	pub port: u16,
	#[structopt(short, long)]
	pub username: Option<String>,
	#[structopt(short, long)]
	pub password: Option<String>,
	#[structopt(short, long,parse(try_from_str = resolve_sock_addr))]
	pub target_addr: SocketAddr,
}
