use std::error::Error;

use crate::Flags;

pub async fn handle_client(
	mut client: tokio::net::TcpStream,
	flags: Flags,
) -> Result<(), Box<dyn Error>>
{
	let auth_line = format!(
		"Proxy-Authorization: Basic {}\r\n",
		base64::encode(&format!(
			"{}:{}",
			flags.username.unwrap_or_default(),
			flags.password.unwrap_or_default()
		))
	);
	unimplemented!()
}
