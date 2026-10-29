mod configs;
mod management;
mod proto;

use anyhow::{Context, Result};
use hyper_util::rt::tokio::TokioIo;
use std::path::Path;
use tokio::net::UnixStream;
use tonic::transport::{Endpoint, Uri};
use tower::service_fn;

pub struct PCWClient {
    pub configs: configs::Configs,
    pub management: management::Management,
}

impl PCWClient {
    pub async fn connect(uds_path: &'static Path) -> Result<Self> {
        let endpoint = Endpoint::from_static("http://localhost");
        let channel = endpoint
            .connect_with_connector(service_fn(move |_: Uri| async move {
                let stream = UnixStream::connect(uds_path).await?;
                Ok::<_, std::io::Error>(TokioIo::new(stream))
            }))
            .await
            .context("server connection error")?;

        Ok(Self {
            configs: configs::Configs::new(channel.clone()),
            management: management::Management::new(channel),
        })
    }
}
