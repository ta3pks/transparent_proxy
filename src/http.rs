use std::error::Error;

use crate::Flags;

pub async fn handle_client(
	mut client: tokio::net::TcpStream,
	flags: Flags,
) -> Result<(), Box<dyn Error>>
{
	unimplemented!()
}
