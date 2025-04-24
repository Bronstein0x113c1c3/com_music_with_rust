use std::{
    io::Read,
    net::{SocketAddr, SocketAddrV4},
};

use rustls::crypto::aws_lc_rs::default_provider;
use tokio::io::AsyncWriteExt;

mod server_util;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    use rustls::crypto::ring::default_provider;
    default_provider().install_default().unwrap();
    //decompress the sound chunk
    let file = std::io::BufReader::new(
        std::fs::File::open("./list_songs/rightforyou_instrumental.mp3").unwrap(),
    );
    // std::io::pipe()
    let mut source = rodio::Decoder::new(file).unwrap();

    let buffer: Vec<i16> = source.by_ref().collect();
    let bytes: Vec<u8> = buffer
        .iter()
        .flat_map(|&num| num.to_be_bytes().to_vec())
        .collect();

    //create server endpoint...
    let addr = SocketAddr::new(
        std::net::IpAddr::V4(std::net::Ipv4Addr::new(0, 0, 0, 0)),
        8080,
    );

    let (endpoint, _) = server_util::make_server_endpoint(addr).unwrap();
    let incoming = endpoint.accept().await.unwrap();
    let mut conn = incoming.await.unwrap();
    let mut uni = conn.open_uni().await.unwrap();
    // uni.
    let mut iter = bytes.chunks(8820);
    while let Some(chunk) = iter.next() {
        // uni.stopped()
        uni.write(chunk).await;
        uni.flush();
    }
    uni.finish();
    uni.stopped().await;
    //send the sound to the client...
    Ok(())
}
