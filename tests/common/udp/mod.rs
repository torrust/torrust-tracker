use torrust_tracker::servers::service;
use torrust_tracker::servers::udp::server::UdpHandle;

pub mod environment;

pub type Started<'a> = environment::Environment<service::Started<UdpHandle>>;
//pub type Stopped<'a> = environment::Environment<service::Stopped>;

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use tokio::time::sleep;
    use torrust_tracker_test_helpers::configuration;

    use crate::common::udp::Started;

    #[tokio::test]
    async fn it_should_make_and_stop_udp_server() {
        let env = Started::new(&configuration::ephemeral().into()).await;
        sleep(Duration::from_secs(1)).await;
        env.stop().await;
        sleep(Duration::from_secs(1)).await;
    }
}
