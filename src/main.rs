use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tracing::{debug, error, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let mut stream = TcpStream::connect("37.187.124.186:1222").await?;
    let (mut read, mut write) = stream.split();

    let mut buf: [u8; 4] = [0; 4];

    let message = [
        0x49, 0x00, 0x00, 0x30, 0x39, 0x00, 0x00, 0x00, 0x65, 0x49, 0x00, 0x00, 0x30, 0x3a, 0x00,
        0x00, 0x00, 0x66, 0x49, 0x00, 0x00, 0x30, 0x3b, 0x00, 0x00, 0x00, 0x64, 0x49, 0x00, 0x00,
        0xa0, 0x00, 0x00, 0x00, 0x00, 0x05, 0x51, 0x00, 0x00, 0x30, 0x00, 0x00, 0x00, 0x40, 0x00,
    ];

    write.write_all(&message).await?;

    while let Ok(n) = read.read(&mut buf).await {
        info!("Stream incoming...");

        if n == 0 {
            info!("End of stream");
            return Ok(());
        }

        let message = i32::from_be_bytes(buf);
        debug!(?message);
        return Ok(());
    }

    error!("Cannot read from socket");
    Err("Could not read from socket".into())
}
