mod client_util;
use client_util::{run_unsafe_client, setup_unsafe};
use tokio::{io::AsyncReadExt, select};
#[tokio::main]
async fn main() {
    //setup connection
    use rustls::crypto::ring::default_provider;
    default_provider().install_default().unwrap();
    let addr = std::net::SocketAddr::new(
        std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)),
        8080,
    );
    let (mut connection, mut endpoint) = setup_unsafe(addr).await.unwrap();
    let mut recv_stream = connection.accept_uni().await.unwrap();
    // sound....
    let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let sink = std::sync::Arc::new(std::sync::Mutex::new(
        rodio::Sink::try_new(&stream_handle).unwrap(),
    ));
    let mut s1 = sink.clone();
    let t1 = tokio::spawn(async move {
        loop {
            let mut buf = [0u8; 8820];
            let res = recv_stream.read(&mut buf).await;
            match res {
                Ok(x) => {
                    match x {
                        Some(n) => {
                            println!("{}: with {}", recv_stream.id(), n);
                            let res: Vec<i16> = buf[..n]
                                .chunks_exact(2)
                                .map(|chunk| i16::from_be_bytes(chunk.try_into().unwrap()))
                                .collect();
                            let res = rodio::buffer::SamplesBuffer::new(2, 48000, res);
                            s1.lock().unwrap().append(res);
                        }
                        None => {
                            println!("eof!!!!");
                            break;
                        }
                    }
                }
                Err(e) => {
                    break;
                }
            }
        }
    });
    // t1.await;
    // sink.lock().unwrap().sleep_until_end();
    // select! {
    //     _ = t1=>{
    //         sink.lock().unwrap().sleep_until_end();
    //     }
    // };
}
