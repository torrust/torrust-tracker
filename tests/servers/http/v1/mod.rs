use std::net::IpAddr;

use reqwest::Response;
use torrust_tracker::core::auth::Key;
use torrust_tracker::servers::http::server::Running;
use torrust_tracker::shared::bit_torrent::tracker::http::client::{requests, Client};
use torrust_tracker_primitives::info_hash::InfoHash;
use torrust_tracker_primitives::peer;

use super::environment::Environment;
use super::TIMEOUT;

pub mod contract;

pub(crate) const PORT: u16 = 17548;

pub(crate) fn create_default_client(env: &Environment<Running>) -> Client {
    let url: url::Url = format!("http://{}/", &env.bind_address())
        .parse()
        .expect("it should make a valid url");
    Client::new(url, TIMEOUT).expect("it should make a client")
}

pub(crate) fn create_bonded_client(env: &Environment<Running>, local_address: IpAddr) -> Client {
    let url: url::Url = format!("http://{}/", &env.bind_address())
        .parse()
        .expect("it should make a valid url");
    Client::bind(url, TIMEOUT, local_address).expect("it should make a client")
}

pub(crate) fn create_authenticated_client(env: &Environment<Running>, key: Key) -> Client {
    let url: url::Url = format!("http://{}/", &env.bind_address())
        .parse()
        .expect("it should make a valid url");
    Client::authenticated(url, TIMEOUT, key).expect("it should make a client")
}

pub(crate) async fn create_client_response(env: &Environment<Running>, path: &str) -> Response {
    create_default_client(env).get(path).await.expect("it should get a response")
}

pub(crate) async fn create_client_announce_response(env: &Environment<Running>, query: &requests::Announce) -> Response {
    create_default_client(env)
        .announce(query)
        .await
        .expect("it should get a response")
}

pub(crate) async fn create_client_scrape_response(env: &Environment<Running>, query: &requests::Scrape) -> Response {
    create_default_client(env)
        .scrape(query)
        .await
        .expect("it should get a response")
}

pub(crate) async fn create_bonded_client_announce_response(
    env: &Environment<Running>,
    local_address: IpAddr,
    query: &requests::Announce,
) -> Response {
    create_bonded_client(env, local_address)
        .announce(query)
        .await
        .expect("it should get a response")
}

pub(crate) async fn create_bonded_client_scrape_response(
    env: &Environment<Running>,
    local_address: IpAddr,
    query: &requests::Scrape,
) -> Response {
    create_bonded_client(env, local_address)
        .scrape(query)
        .await
        .expect("it should get a response")
}

pub(crate) fn create_announce_query<I: Into<InfoHash>, P: Into<peer::Id>>(
    info_hash: I,
    peer_id: P,
) -> requests::announce::QueryBuilder {
    requests::announce::QueryBuilder::new(info_hash.into(), peer_id.into(), PORT)
}

pub(crate) fn create_scrape_query<I: Into<requests::scrape::QueryBuilder>>(
    info_hash: Option<I>,
) -> requests::scrape::QueryBuilder {
    if let Some(info_hash) = info_hash {
        info_hash.into()
    } else {
        requests::scrape::QueryBuilder::default()
    }
}

pub(crate) fn create_default_announce_prams() -> requests::announce::QueryParams {
    (&create_announce_query(1, 1).build()).into()
}
