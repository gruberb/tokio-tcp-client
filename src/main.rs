use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tracing::{debug, error, info};

const IP_ADDRESS: &str = "0.0.0.0";
const PORT: u16 = 6374;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let mut stream = TcpStream::connect(format!("{IP_ADDRESS}:{PORT}")).await?;
    let (mut read, mut write) = stream.split();

    let mut buf: [u8; 4] = [0; 4];

    // Byte message for protohackers.com problem #2 - "Means to an End""
    let message = [
        0x49, 0x00, 0x00, 0x30, 0x39, 0x00, 0x00, 0x00, 0x65, 0x49, 0x00, 0x00, 0x30, 0x3a, 0x00,
        0x00, 0x00, 0x66, 0x49, 0x00, 0x00, 0x30, 0x3b, 0x00, 0x00, 0x00, 0x64, 0x49, 0x00, 0x00,
        0xa0, 0x00, 0x00, 0x00, 0x00, 0x05, 0x51, 0x00, 0x00, 0x30, 0x00, 0x00, 0x00, 0x40, 0x00,
    ];

    write.write_all(&message).await?;

    if let Ok(n) = read.read_exact(&mut buf).await {
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
