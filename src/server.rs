use sellershut_core::categories::{
    mutate_categories_server::MutateCategoriesServer,
    query_categories_server::QueryCategoriesServer,
};
use tonic::transport::{Server, server::TcpIncoming};
use tracing::info;

use crate::state::AppState;

pub async fn serve(state: AppState, tx: tokio::sync::oneshot::Sender<u16>) -> anyhow::Result<()> {
    let listener = tokio::net::TcpListener::bind(state.addr).await?;

    let socket_addr = listener
        .local_addr()
        .expect("should get socket_addr from listener");

    tx.send(socket_addr.port())
        .expect("port channel to be open");

    info!(addr = ?socket_addr, "starting server");

    Server::builder()
        .trace_fn(|_| tracing::info_span!(env!("CARGO_PKG_NAME")))
        .add_service(QueryCategoriesServer::new(state.clone()))
        .add_service(MutateCategoriesServer::new(state))
        .serve_with_incoming(TcpIncoming::from_listener(listener, true, None).expect("listener"))
        .await?;

    Ok(())
}
