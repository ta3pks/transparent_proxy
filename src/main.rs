use std::process::exit;

use structopt::StructOpt;

use tokio::net::TcpListener;
use transparent_proxy::{Flags, TargetType};
#[tokio::main]
async fn main() {
    let flags = Flags::from_args();
    if (flags.username.is_none() && flags.password.is_some())
        || (flags.username.is_some() && flags.password.is_none())
    {
        eprintln!("username is required when password is provided or vice versa");
        exit(1);
    }
    check_features(&flags.target_type);
    println!("{:#?}", flags);
    let listener = TcpListener::bind((flags.host, flags.port)).await.unwrap();
    while let Ok((sock, addr)) = listener.accept().await {
        let flags = flags.clone();
        match flags.target_type {
            TargetType::Socks5 => {
                #[cfg(feature = "socks5")]
                tokio::spawn(async move {
                    println!("new socks5 conn from {addr}");
                    if let Err(e) = transparent_proxy::socks::handle_client(sock, flags).await {
                        eprintln!("{}", e);
                    }
                });
            }

            TargetType::Http => {
                #[cfg(feature = "http")]
                tokio::spawn(async move {
                    println!("new http conn from {addr}");
                    if let Err(e) = transparent_proxy::http::handle_client(sock, flags).await {
                        eprintln!("{}", e);
                    }
                });
            }
        }
    }
}
fn check_features(tp: &TargetType) {
    match tp {
        TargetType::Socks5 => {
            #[cfg(not(feature = "socks5"))]
            {
                eprintln!("socks5 feature is not enabled during build");
                exit(1);
            }
        }
        TargetType::Http => {
            #[cfg(not(feature = "http"))]
            {
                eprintln!("http feature is not enabled during build");
                exit(1);
            }
        }
    }
}
