use rodio::{Decoder, OutputStream, Sink};
use std::{
    // io::{self, BufRead},
    fs::read, net::{self, SocketAddr}, sync::{Arc, Mutex}, time::Duration
};
use tokio::signal;
use tokio::{
    io::{self, AsyncBufReadExt, BufReader},
    time::timeout,
};
use tokio::{
    net::UdpSocket,
    select,
    sync::mpsc::{self, Receiver, UnboundedReceiver, UnboundedSender, channel, unbounded_channel},
    time::sleep,
};
use tokio_util::sync::CancellationToken;

async fn main2() {
    /* */
    let (stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Arc::new(Mutex::new(Sink::try_new(&stream_handle).unwrap()));
    let (mut tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<Vec<u8>>();
    // Add a oneshot channel for completion notification
    let (complete_tx, complete_rx) = tokio::sync::oneshot::channel();

    // let file = std::io::BufReader::new(
    //     std::fs::File::open("./list_songs/01 - There's Nothing Holdin' Me Back.flac").unwrap(),
    // );
    // let mut source = Decoder::new(file).unwrap();

    // // Producer task
    // let t = tokio::spawn(async move {
    //     let buffer: Vec<i16> = source.by_ref().collect();
    //     let bytes: Vec<u8> = buffer
    //         .iter()
    //         .flat_map(|&num| num.to_be_bytes().to_vec())
    //         .collect();
    //     let mut iter = bytes.chunks(16384);

    //     while let Some(res) = iter.next() {
    //         let mut salt = "music_chunk".as_bytes().to_vec();
    //         let mut res = res.to_owned();
    //         res.append(&mut salt);
    //         tx.send(res);
    //     }

    //     // Signal that we've finished sending all data
    // });

    // Consumer task
    let s1 = sink.clone();
    let t3 = tokio::spawn(async move {
        while let Some(res) = rx.recv().await {
            println!("receiving chunk...");
            let res: Vec<i16> = res[..res.len() - 1]
                .chunks_exact(2)
                .map(|chunk| i16::from_be_bytes(chunk.try_into().unwrap()))
                .collect();
            let res = rodio::buffer::SamplesBuffer::new(2, 48000, res);
            s1.lock().unwrap().append(res);
        }
        complete_tx.send(()).unwrap();
    });

    // Player wait task
    let t2 = tokio::spawn(async move {
        // Wait for either:
        // 1. The completion signal (all data sent)
        // 2. The consumer task to finish (in case of error)
        tokio::select! {
            _ = complete_rx => {
                println!("All data received, waiting for playback to finish...");
                sink.lock().unwrap().sleep_until_end();
                println!("Playback completed!");
            }
            // _ = t3 => {
            //     println!("Consumer task finished early");
            // }
        }
    });

    // Wait for all tasks to complete
    tokio::try_join!(t3, t2).unwrap();
}

#[tokio::main]
async fn main() {
    let mut client_socket = tokio::net::UdpSocket::bind("0.0.0.0:0").await.unwrap();
    let mut client_socket = Arc::new(client_socket);
    let remote_addr = "0.0.0.0:8080".parse::<SocketAddr>().unwrap();
    client_socket.connect(remote_addr).await.unwrap();

    // for network_ block
    let mut client_socket_network_block = client_socket.clone();
    // let mut addr_network_block = remote_addr.clone();

    println!("init done, creating control block!!!");
    client_socket.send("".as_bytes()).await;
    let mut list_song = vec![];
    loop {
        let mut buf = vec![];
        client_socket.recv_buf(&mut buf).await;
        // println!("{}",buf.len());
        if buf.len() == 0 {
            break;
        }
        list_song.push(String::from_utf8(buf).unwrap());
    }
    for i in 0..list_song.len() {
        println!("{}: {}", i, list_song[i]);
    }
    //control -> network -> audio
    let (control, mut receiver) = control_block();
    let (mut data_rx, network_loop) = network_block(receiver, client_socket_network_block);
    // let (audio_loop, mut sink,cancellation) = audio_block(data_rx);
    let t1 = tokio::spawn(control);
    let t2 = tokio::spawn(network_loop);
    audio_block(data_rx).await;
    // let t3 = tokio::spawn(audio_loop);
    // let t4 = tokio::spawn(async move {
    //     // Wait for either:
    //     // 1. The completion signal (all data sent)
    //     // 2. The consumer task to finish (in case of error)
    //     tokio::select! {
    //         _ = cancellation.cancelled() => {
    //             println!("All data received, waiting for playback to finish...");
    //             sink.lock().unwrap().sleep_until_end();
    //             println!("Playback completed!");
    //         }
    //         // _ = t3 => {
    //         //     println!("Consumer task finished early");
    //         // }
    //     }
    // });
    //    tokio::try_join!(t1,t2,t3,t4).unwrap();

    // let (stream, stream_handle) = OutputStream::try_default().unwrap();
    // let sink = Arc::new(Mutex::new(Sink::try_new(&stream_handle).unwrap()));
}
fn control_block() -> (
    impl Future<Output = ()>,
    Receiver<Vec<u8>>,
    // CancellationToken,
    // CancellationToken,
) {
    let (mut song_request_sender, mut song_request_receiver) = channel(1);
    // let cancellation = CancellationToken::new();
    // let audio_cancellation = cancellation.clone();
    // let network_cancellation = cancellation.clone();

    // let command = async move {
    //     let mut input_string = String::new();
    //     loop {
    //         println!("which song do you want to play????: ");
    //         io::stdin().read_line(&mut input_string).unwrap();
    //         println!("{:?}", input_string);
    //         if input_string.as_str().contains("exit") {
    //             println!("caught exit");
    //             drop(song_request_sender);
    //             break;
    //         }
    //         let song = input_string.as_bytes();
    //         let song = &song[..song.len() - 1];
    //         song_request_sender.send(song.to_vec());
    //     }
    //     // loop{
    //         // song_request_sender.send("82".as_bytes().to_vec());
    //         // signal::ctrl_c().await;
    //     // }
    // };
    let command = async move {
        // let stdin = io::stdin();
        // let mut reader = io::BufReader::new(stdin).lines();

        // println!("Which song do you want to play? (type 'exit' to quit):");
        let stdin = io::stdin();
        let mut reader = io::BufReader::new(stdin).lines();

        println!("Type something and press Enter (Ctrl+D to exit):");
        // while let Ok(Some(line)) = reader.next_line().await {
        //     println!("You typed: {}", line);
        // }
        // drop(reader);
        loop {
            select! {
                _= signal::ctrl_c()=>{
                    drop(song_request_sender);
                    // break;
                    drop(reader);
                    return;
                },
                Ok(Some(line)) = reader.next_line()=>{
                    
                    println!("You entered: {:?}", line);

                    if line.contains("exit") {
                        println!("Caught exit command");
                        // drop(song_request_sender);
                        break;
                    }

                    // Send the song bytes (excluding newline)
                    let song_bytes = line.as_bytes().to_vec();
                    if let Err(e) = song_request_sender.send(song_bytes).await {
                        eprintln!("Failed to send song request: {}", e);
                        break;
                    }
                }
            }
        }

        // drop(song_request_sender); // Explicit cleanup
    };
    (
        command,
        song_request_receiver,
        // audio_cancellation,
        // network_cancellation,
    )
}

fn network_block(
    // socket: Arc<UdpSocket>,
    // addr: SocketAddr,
    mut song_request_receiver: Receiver<Vec<u8>>,
    // cancellation: CancellationToken,
    client_socket: Arc<UdpSocket>,
) -> (UnboundedReceiver<Vec<u8>>, impl Future<Output = ()>) {
    // cancellation.cancelled()
    let (chunk_tx, mut chunk_rx) = unbounded_channel();
    let network_loop = async move {
        while let Some(res) = song_request_receiver.recv().await {
            println!("received");
            client_socket.send(&res).await;
            let mut seq = [115, 101, 113, 0];
            loop {
                let mut buf = Vec::with_capacity(1025);
                client_socket.send(&seq).await;
                if let Err(_) = tokio::time::timeout(
                    tokio::time::Duration::from_secs(60),
                    client_socket.recv_buf(&mut buf),
                )
                .await
                {
                    // println!("out of time!!!!");
                    // drop(sender);
                    //reset
                    break;
                }
                // println!("{}", buf.len());
                // size += (buf.len() - 1);
                chunk_tx.send(buf[..buf.len() - 1].to_vec());
                // let res = rodio::buffer::SamplesBuffer::new(2, 48000, res);
                // s1.lock().unwrap().append(res);
                //         tx.send(res).await;
                if buf[buf.len() - 1] == seq[3] {
                    // println!("received!!! {}", seq[3]);
                    seq[3] = adding(seq[3]);
                }
            }
        }
        println!("control block done, dropping audio block");
        drop(chunk_tx);
    };
    (chunk_rx, network_loop)
}

async fn audio_block(mut chunk_rx: UnboundedReceiver<Vec<u8>>)
/*  ) -> (impl Future<Output = ()>, Arc<Mutex<Sink>>, CancellationToken) */
{
    let cancellation = CancellationToken::new();
    let cancellation_clone = cancellation.clone();
    let (stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Arc::new(Mutex::new(Sink::try_new(&stream_handle).unwrap()));
    let returned_sink = sink.clone();
    let audio_work = async move {
        // let mut data:Vec<i16> =vec![];
        // let mut total: usize = 0;
        while let Some(buf) = chunk_rx.recv().await {
            // println!("got from control block");
            // chunk_rx.
            let mut res: Vec<i16> = buf
                .chunks_exact(2)
                .map(|chunk| i16::from_be_bytes(chunk.try_into().unwrap()))
                .collect();
            let res = rodio::buffer::SamplesBuffer::new(2, 44100, res);
            sink.lock().unwrap().append(res);
            // println!("len of {}", res.len());
            // data.append(&mut res);
            // total += res.len();
        }
        println!("audio work stopped!!!");
        cancellation.cancel();
        // return data;
    };
    let waiting = async move {
        tokio::select! {
            _ = cancellation_clone.cancelled()=>{
                // let returned_sink = sink.clone();
                returned_sink.lock().unwrap().sleep_until_end();
                println!("playback completed");
            }
        }
    };
    tokio::try_join!(tokio::spawn(audio_work), tokio::spawn(waiting)).unwrap();
    // (audio_work,returned_sink,cancellation_clone)
}

// control block, network block, audio block
fn adding(i: u8) -> u8 {
    if i == 255 {
        return 0;
    } else {
        return i + 1;
    }
}
