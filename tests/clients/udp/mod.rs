// use futures::future::BoxFuture;
// use futures::FutureExt;
// use rstest::{fixture, rstest};
// use torrust_tracker::shared::bit_torrent::tracker;
// use torrust_tracker_configuration::CLIENT_TIMEOUT_DEFAULT;
// use torrust_tracker_test_helpers::configuration;

// use crate::common::udp::Started;

// type Server<'b, 'a> = BoxFuture<'a, Started<'b>>;
// type ClientFuture<'a> = BoxFuture<'a, tracker::udp::Client>;
// type Client<'a> = BoxFuture<'a, dyn (Fn(&Started<'a>) -> ClientFuture<'a>)>;

// #[fixture]
// fn server<'a>() -> Server<'a> {
//     let a = Started::new(&configuration::ephemeral().into()).boxed();

//     a
// }

// #[fixture]
// fn client() -> Client<'static> {
//     |server| async move {
//         tracker::udp::Client::connect(server.bind_address(), CLIENT_TIMEOUT_DEFAULT)
//             .await
//             .unwrap()
//     }
// }

// #[rstest]
// #[tokio::test]
// async fn it_should_send_and_receive(server: Server<'_>, client: Client<'_>) {
//     //let server =

//     let client = client(&server.await).await;

//     client.check().await.expect("it should run check successfully");

//     //drop(server.tracker.get_stats().await);
// }
