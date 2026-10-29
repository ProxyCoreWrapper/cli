use super::proto;
use proto::log_record::FileDescriptor;
use tonic::{Result, transport::Channel};

pub struct Management {
    client: proto::management_client::ManagementClient<Channel>,
}

impl Management {
    pub fn new(channel: Channel) -> Self {
        Self {
            client: proto::management_client::ManagementClient::new(channel),
        }
    }

    pub async fn start(&mut self, config_id: String) -> Result<()> {
        let request = tonic::Request::new(proto::StartRequest { config_id });
        self.client.start(request).await?;
        Ok(())
    }

    pub async fn restart(&mut self, config_id: String) -> Result<()> {
        let request = tonic::Request::new(proto::RestartRequest { config_id });
        self.client.restart(request).await?;
        Ok(())
    }

    pub async fn stop(&mut self, config_id: String) -> Result<()> {
        let request = tonic::Request::new(proto::StopRequest { config_id });
        self.client.stop(request).await?;
        Ok(())
    }

    pub async fn print_log_stream(&mut self, config_id: String) -> Result<()> {
        let request = tonic::Request::new(proto::ReceiveLogRequest { config_id });
        let mut stream = self.client.receive_log(request).await?.into_inner();

        while let Some(response) = stream.message().await? {
            match response.fd() {
                FileDescriptor::Out => print!("{}", response.log),
                FileDescriptor::Err => eprint!("{}", response.log),
            }
        }

        Ok(())
    }
}
