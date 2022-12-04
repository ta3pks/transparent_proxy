use std::{
    net::{IpAddr, SocketAddr, ToSocketAddrs},
    str::FromStr,
};

use structopt::StructOpt;

fn resolve_sock_addr(addr: &str) -> Result<SocketAddr, String> {
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
pub struct Flags {
    #[structopt(short, long, default_value = "127.0.0.1")]
    pub host: IpAddr,
    #[structopt(short = "P", long, default_value = "55000")]
    pub port: u16,
    #[structopt(short, long)]
    ///If username contains %R, it will be replaced with a random number between 1 and u32::MAX
    pub username: Option<String>,
    #[structopt(short, long)]
    pub password: Option<String>,
    #[structopt(short, long,parse(try_from_str = resolve_sock_addr))]
    pub target_addr: SocketAddr,
    #[structopt(short = "T", long, default_value = "socks5")]
    /// socks5, http
    pub target_type: TargetType,
}
#[derive(Debug, Clone)]
pub enum TargetType {
    Socks5,
    Http,
}
impl FromStr for TargetType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "socks5" => Ok(TargetType::Socks5),
            "http" => Ok(TargetType::Http),
            _ => Err(format!(
                "invalid target type: {s}\nValid types: socks5, http"
            )),
        }
    }
}
