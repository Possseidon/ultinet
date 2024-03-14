use std::{
    env::args,
    net::{Ipv4Addr, SocketAddr},
};

use anyhow::{bail, Result};
use ultinet::net::{
    client::{Client, Compatibility},
    server::Server,
    BasicLogConnectionHandler, DefaultConnectionHandler,
};

fn main() -> Result<()> {
    match args().nth(1).as_deref() {
        Some("server") => server(),
        Some("client") => client(),
        _ => bail!("expected 'server' or 'client'"),
    }
}

fn server() -> Result<()> {
    let mut server = <Server>::host((Ipv4Addr::LOCALHOST, 42069))?;
    loop {
        server.update(&mut BasicLogConnectionHandler);
        // std::thread::sleep(Duration::from_millis(100));
    }
}

fn client() -> Result<()> {
    let mut client = <Client>::new()?;

    let server_addr = SocketAddr::from((Ipv4Addr::LOCALHOST, 42069));
    client.listen(server_addr)?;
    while let Some(Compatibility::Pending) = client.compatibility(server_addr) {
        client.update(&mut BasicLogConnectionHandler);
        // std::thread::sleep(Duration::from_millis(1));
    }

    if let Some(compatibility) = client.compatibility(server_addr) {
        match compatibility {
            Compatibility::Incompatible(error) => {
                println!("incompatible: {error}");
                return Ok(());
            }
            Compatibility::Pending => unreachable!(),
            Compatibility::Compatible => {}
        }
    } else {
        println!("error");
        return Ok(());
    }

    let query_packet = client.query(server_addr, ())?;
    println!("query: {query_packet:?}");
    let connect_packet = client.connect(server_addr, ())?;
    println!("connect: {connect_packet:?}");

    loop {
        client.update(&mut DefaultConnectionHandler);
        // std::thread::sleep(Duration::from_millis(1));
    }
}