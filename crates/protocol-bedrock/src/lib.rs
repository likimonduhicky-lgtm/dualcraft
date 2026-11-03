use common::ServerState;
use tokio::net::UdpSocket;
use tracing::{info, warn};

/// Start the Bedrock Edition UDP (RakNet) listener.
pub async fn listen(addr: &str, state: ServerState) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let socket = UdpSocket::bind(addr).await?;
    info!("Bedrock listener bound to {}", addr);

    let mut buf = vec![0u8; 2048];

    loop {
        let (len, remote) = socket.recv_from(&mut buf).await?;
        let data = &buf[..len];

        // RakNet unconnected ping starts with 0x01
        if !data.is_empty() && data[0] == 0x01 {
            info!("Bedrock unconnected ping from {}", remote);
            // TODO: send unconnected pong with MOTD, protocol version, player counts, etc.
            let (online, max) = state.player_count().await;
            let _ = (online, max); // silence unused for now
        } else {
            warn!("Bedrock packet from {} ({} bytes) — not handled yet", remote, len);
        }
    }
}
