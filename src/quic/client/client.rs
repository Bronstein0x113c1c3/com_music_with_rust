mod client_util;
use std::time::Duration;

use client_util::{run_unsafe_client, setup_unsafe};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    select,
    sync::mpsc::unbounded_channel,
};
use tokio_util::sync::CancellationToken;
#[tokio::main]
async fn main() {
    //setup connection
    use rustls::crypto::ring::default_provider;
    default_provider().install_default().unwrap();
    
    let addr = std::net::SocketAddr::new(
        std::net::IpAddr::V4(std::net::Ipv4Addr::new(192, 168, 0, 102)),
        8080,
    );
    let (mut connection, mut endpoint) = setup_unsafe(addr).await.unwrap();
    let (mut send_stream, mut recv_stream) = connection.accept_bi().await.unwrap();
    // sound....
    let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let sink = std::sync::Arc::new(std::sync::Mutex::new(
        rodio::Sink::try_new(&stream_handle).unwrap(),
    ));
    let cancellation = CancellationToken::new();
    let c = cancellation.clone();
    let mut s1 = sink.clone();
    let (sender, mut receiver) = unbounded_channel();
    let t1 = tokio::spawn(async move {
        loop {
            let mut buf = [0u8; 1000];
            let res = recv_stream.read_exact(&mut buf).await;
            match res {
                Ok(x) => {
                    // println!("{}: with {}", recv_stream.id(), n);
                    let res: Vec<i16> = buf
                        .chunks_exact(2)
                        .map(|chunk| i16::from_be_bytes(chunk.try_into().unwrap()))
                        .collect();
                    let res = rodio::buffer::SamplesBuffer::new(2, 40000, res);
                    // s1.lock().unwrap().append(res);
                    sender.send(res);
                    // let wait_buffer = [0u8; 1];
                    
                    // send_stream.write_all(&wait_buffer).await;
                    // tokio::time::sleep(Duration::from_millis(100)).await;
                    // send_stream.flush().await;
                }

                Err(e) => {
                    println!("last piece of the song!!!!");
                    let res: Vec<i16> = buf
                        .chunks_exact(2)
                        .map(|chunk| i16::from_be_bytes(chunk.try_into().unwrap()))
                        .collect();
                    let res = rodio::buffer::SamplesBuffer::new(2, 40000, res);
                    // s1.lock().unwrap().append(res);
                    sender.send(res);
                    break;
                }
            }
            
        }
        cancellation.cancel();
    });
    let t3 = tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(500)).await;
        while let Some(res) = receiver.recv().await {
            s1.lock().unwrap().append(res);
        }
    });
    let t2 = tokio::spawn(async move {
        select! {
            _ = c.cancelled()=>{
                sink.lock().unwrap().sleep_until_end();
            }
        }
    });
    tokio::try_join!(t1, t2, t3).unwrap();
    // t1.await;
    // sink.lock().unwrap().sleep_until_end();
    // select! {
    //     _ = t1=>{
    //         sink.lock().unwrap().sleep_until_end();
    //     }
    // };
}
