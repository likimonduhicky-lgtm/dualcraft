use common::ServerState;
use tokio::net::{TcpListener, TcpStream};
use tracing::{info, warn};

/// Start the Java Edition TCP listener.
pub async fn listen(addr: &str, state: ServerState) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let listener = TcpListener::bind(addr).await?;
    info!("Java listener bound to {}", addr);

    loop {
        let (socket, remote) = listener.accept().await?;
        info!("Java connection from {}", remote);

        let state = state.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_client(socket, state).await {
                warn!("Java client error: {}", e);
            }
        });
    }
}

async fn handle_client(
    mut _socket: TcpStream,
    state: ServerState,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // TODO:
    // 1. Read VarInt packet length + packet id
    // 2. Handle Handshake (protocol version, next state)
    // 3. Status request / login start
    // 4. Encryption + compression (modern versions)
    // 5. Transition to Play state

    let (online, max) = state.player_count().await;
    let motd = state.motd().await;
    info!("Would answer status: {}/{} — {}", online, max, motd);

    // For now just close the connection after logging.
    Ok(())
}
