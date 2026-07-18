use super::proto;
use tonic::{Result as TonicResult, transport::Channel};

pub struct Configs {
    client: proto::configs_client::ConfigsClient<Channel>,
}

impl Configs {
    pub fn new(channel: Channel) -> Self {
        Self {
            client: proto::configs_client::ConfigsClient::new(channel),
        }
    }

    pub async fn list(&mut self) -> TonicResult<Vec<String>> {
        self.client
            .list_configs(proto::ListConfigsRequest::default())
            .await
            .map(|response| response.into_inner().configs_ids)
    }

    pub async fn get(&mut self, config_id: String) -> TonicResult<String> {
        self.client
            .get_config(proto::GetConfigRequest { config_id })
            .await
            .map(|response| response.into_inner().config)
    }

    pub async fn edit(&mut self, config_id: String, config: String) -> TonicResult<()> {
        self.client
            .edit_config(proto::EditConfigRequest { config_id, config })
            .await
            .map(|_| ())
    }
}
