use axum::Router;
use std::net::SocketAddr;
use tokio::{net::TcpListener, signal};

pub async fn serve(addr: SocketAddr, router: Router) -> std::io::Result<()> {
    let listener = TcpListener::bind(addr).await?;
    tracing::info!(%addr, "e_school démarré");
    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown())
        .await
}

/// Arrêt propre sur Ctrl+C ou SIGTERM (docker stop).
async fn shutdown() {
    let ctrl_c = async { signal::ctrl_c().await.ok() };
    #[cfg(unix)]
    let term = async {
        if let Ok(mut s) = signal::unix::signal(signal::unix::SignalKind::terminate()) {
            s.recv().await;
        }
    };
    #[cfg(not(unix))]
    let term = std::future::pending::<()>();
    tokio::select! { _ = ctrl_c => {}, () = term => {} }
    tracing::info!("arrêt en cours");
}
