pub mod entity;
pub mod server;
pub mod state;
pub mod utils;

use serde::Deserialize;
use state::AppState;
use tracing::{debug, trace};

#[derive(Deserialize)]
pub struct AppConfig {
    something: String,
}

pub async fn run(state: AppState, tx: tokio::sync::oneshot::Sender<u16>) -> anyhow::Result<()> {
    trace!("running migrations");
    sqlx::migrate!("./migrations")
        .run(&state.services.postgres)
        .await?;
    debug!("ran migrations");

    server::serve(state, tx).await
}
