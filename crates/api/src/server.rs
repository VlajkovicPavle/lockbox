use std::mem::take;

use anyhow::{Context, Result};
use axum::{Router, serve};
use tokio::{net::TcpListener, signal};

use crate::routes::HealthRoute;

pub struct Server {
    listener: tokio::net::TcpListener,
    router: Router,
}

impl Server {
    pub async fn new(port: u32, addres: String) -> Result<Self> {
        let server_addres = format!("{}:{}", addres, port);
        let listener = TcpListener::bind(server_addres).await?;
        let server = Server {
            listener,
            router: Router::new(),
        };
        Ok(server)
    }
    pub async fn run(mut self) -> Result<()> {
        self.router = take(&mut self.router).merge(HealthRoute::router());
        serve(self.listener, self.router)
            .with_graceful_shutdown(Self::shutdown())
            .await
            .context("->> [run] Server failed to run")?;
        Ok(())
    }
    pub async fn shutdown() {
        signal::ctrl_c()
            .await
            .expect("->> [shutdown_signal] Failed to call shutdown_signal");
    }
}
