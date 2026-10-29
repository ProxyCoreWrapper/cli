use super::proto;
use tonic::{Result, transport::Channel};

pub struct Configs {
    client: proto::configs_client::ConfigsClient<Channel>,
}

impl Configs {
    pub fn new(channel: Channel) -> Self {
        Self {
            client: proto::configs_client::ConfigsClient::new(channel),
        }
    }

    pub async fn list(&mut self) -> Result<Vec<proto::Config>> {
        let mut stream = self
            .client
            .list_configs(proto::ListConfigsRequest::default())
            .await
            .map(|response| response.into_inner())?;

        let mut configs = Vec::new();

        while let Some(config) = stream.message().await? {
            configs.push(config);
        }

        Ok(configs)
    }

    pub async fn get(&mut self, config_id: String) -> Result<String> {
        self.client
            .get_config(proto::GetConfigRequest { config_id })
            .await
            .map(|response| response.into_inner().config)
    }

    pub async fn edit(&mut self, config_id: String, config: Option<String>, core_tag: Option<String>) -> Result<()> {
        self.client
            .edit_config(proto::EditConfigRequest { config_id, config, core_tag })
            .await
            .map(|_| ())
    }
}
