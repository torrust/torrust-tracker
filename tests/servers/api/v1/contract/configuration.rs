use torrust_tracker_test_helpers::configuration;

use crate::servers::api::test_environment::stopped_test_environment;

#[tokio::test]
#[should_panic = "Could not receive bind_address."]
async fn should_fail_with_ssl_enabled_and_bad_ssl_config() {
    let mut test_env = stopped_test_environment(configuration::ephemeral());

    let cfg = test_env.config_mut();

    cfg.ssl_enabled = true;
    cfg.ssl_key_path = Some("bad key path".to_string());
    cfg.ssl_cert_path = Some("bad cert path".to_string());

    test_env.start().await;
}
