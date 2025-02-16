use categories_service::state::AppState;
use sellershut_core::categories::{
    mutate_categories_client::MutateCategoriesClient,
    query_categories_client::QueryCategoriesClient,
};
use sellershut_services::{tracing::TracingBuilder, Services};
use sqlx::PgPool;
use tokio::sync::oneshot;
use tonic::transport::Channel;

use std::sync::Once;

static TRACING: Once = Once::new();

pub struct TestApp {
    state: AppState,
    pub query: QueryCategoriesClient<Channel>,
    pub mutate: MutateCategoriesClient<Channel>,
}

impl TestApp {
    pub async fn new(pool: PgPool) -> Self {
        let (tx, rx) = oneshot::channel();
        // Set port to 0 so tests can spawn multiple servers on OS assigned ports.

        // Setup tracing. Once.
        TRACING.call_once(|| {
            TracingBuilder::new().build(Some("warn".into()));
        });

        let services = Services { postgres: pool };
        let state = AppState::new(0, services);

        dbg!(&state.addr.port());

        tokio::spawn(categories_service::run(state.clone(), tx));
        let port = rx.await.expect("channel to be open");
        let addr = format!("http://[::1]:{port}");

        let (query_client, mutation_client) = tokio::try_join!(
            QueryCategoriesClient::connect(addr.to_string()),
            MutateCategoriesClient::connect(addr)
        )
        .expect("expect server to be running");

        Self {
            state,
            query: query_client,
            mutate: mutation_client,
        }
    }
}
