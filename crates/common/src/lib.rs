use std::sync::Arc;
use tokio::sync::RwLock;

/// Very small shared server state used by both protocol handlers.
#[derive(Debug, Default)]
pub struct ServerStateInner {
    pub online_players: u32,
    pub max_players: u32,
    pub motd: String,
}

#[derive(Clone)]
pub struct ServerState {
    inner: Arc<RwLock<ServerStateInner>>,
}

impl ServerState {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(ServerStateInner {
                online_players: 0,
                max_players: 20,
                motd: "DualCraft - Java + Bedrock".to_string(),
            })),
        }
    }

    pub async fn player_count(&self) -> (u32, u32) {
        let guard = self.inner.read().await;
        (guard.online_players, guard.max_players)
    }

    pub async fn motd(&self) -> String {
        self.inner.read().await.motd.clone()
    }
}

impl Default for ServerState {
    fn default() -> Self {
        Self::new()
    }
}
