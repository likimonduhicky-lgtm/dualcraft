use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("DualCraft starting...");
    info!("Java Edition   → TCP  :25565");
    info!("Bedrock Edition → UDP :19132");

    // Shared state (placeholder)
    let state = common::ServerState::new();

    // Start both listeners concurrently
    let java = tokio::spawn(protocol_java::listen("0.0.0.0:25565", state.clone()));
    let bedrock = tokio::spawn(protocol_bedrock::listen("0.0.0.0:19132", state.clone()));

    // Wait for either to finish (they shouldn't under normal conditions)
    tokio::select! {
        res = java => {
            if let Err(e) = res {
                tracing::error!("Java listener task failed: {}", e);
            }
        }
        res = bedrock => {
            if let Err(e) = res {
                tracing::error!("Bedrock listener task failed: {}", e);
            }
        }
    }

    Ok(())
}
