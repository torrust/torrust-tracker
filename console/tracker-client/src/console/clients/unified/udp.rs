use std::net::{Ipv4Addr, SocketAddr, ToSocketAddrs};
use std::str::FromStr;

use anyhow::Context;
use bittorrent_primitives::info_hash::InfoHash as TorrustInfoHash;
use bittorrent_udp_tracker_protocol::{AnnounceEvent, Response, TransactionId};
use clap::{Subcommand, ValueEnum};
use torrust_tracker_configuration::DEFAULT_TIMEOUT;
use url::Url;

use super::app::OutputFormat;
use crate::console::clients::udp::checker::AnnounceParams;
use crate::console::clients::udp::responses::dto::SerializableResponse;
use crate::console::clients::udp::responses::json::ToJson;
use crate::console::clients::udp::{checker, Error};

const RANDOM_TRANSACTION_ID: i32 = -888_840_697;

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum CliAnnounceEvent {
    None,
    Completed,
    Started,
    Stopped,
}

impl From<CliAnnounceEvent> for AnnounceEvent {
    fn from(value: CliAnnounceEvent) -> Self {
        match value {
            CliAnnounceEvent::None => AnnounceEvent::None,
            CliAnnounceEvent::Completed => AnnounceEvent::Completed,
            CliAnnounceEvent::Started => AnnounceEvent::Started,
            CliAnnounceEvent::Stopped => AnnounceEvent::Stopped,
        }
    }
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Announce {
        #[arg(value_parser = parse_socket_addr)]
        tracker_socket_addr: SocketAddr,
        #[arg(value_parser = parse_info_hash)]
        info_hash: TorrustInfoHash,
        #[arg(long)]
        event: Option<CliAnnounceEvent>,
        #[arg(long)]
        uploaded: Option<u64>,
        #[arg(long)]
        downloaded: Option<u64>,
        #[arg(long)]
        left: Option<u64>,
        #[arg(long, value_parser = parse_non_zero_port)]
        port: Option<u16>,
        #[arg(long = "ip-address")]
        ip_address: Option<Ipv4Addr>,
        #[arg(long = "peer-id", value_parser = parse_peer_id)]
        peer_id: Option<[u8; 20]>,
        #[arg(long)]
        key: Option<i32>,
        #[arg(long = "peers-wanted")]
        peers_wanted: Option<i32>,
        #[arg(long, value_enum, default_value_t = OutputFormat::Json)]
        format: OutputFormat,
    },
    Scrape {
        #[arg(value_parser = parse_socket_addr)]
        tracker_socket_addr: SocketAddr,
        #[arg(value_parser = parse_info_hash, num_args = 1..=74, value_delimiter = ' ')]
        info_hashes: Vec<TorrustInfoHash>,
        #[arg(long, value_enum, default_value_t = OutputFormat::Json)]
        format: OutputFormat,
    },
}

/// # Errors
///
/// Returns an error if the command fails.
pub async fn run(command: Command) -> anyhow::Result<()> {
    let (response, output_format) = match command {
        Command::Announce {
            tracker_socket_addr: remote_addr,
            info_hash,
            event,
            uploaded,
            downloaded,
            left,
            port,
            ip_address,
            peer_id,
            key,
            peers_wanted,
            format,
        } => {
            let params = AnnounceParams {
                event: event.map(Into::into),
                uploaded: uploaded
                    .map(i64::try_from)
                    .transpose()
                    .context("--uploaded value is too large to fit in i64")?,
                downloaded: downloaded
                    .map(i64::try_from)
                    .transpose()
                    .context("--downloaded value is too large to fit in i64")?,
                left: left
                    .map(i64::try_from)
                    .transpose()
                    .context("--left value is too large to fit in i64")?,
                port,
                ip_address,
                peer_id,
                key,
                peers_wanted,
            };
            (handle_announce(remote_addr, &info_hash, &params).await?, format)
        }
        Command::Scrape {
            tracker_socket_addr: remote_addr,
            info_hashes,
            format,
        } => (handle_scrape(remote_addr, &info_hashes).await?, format),
    };

    let response: SerializableResponse = response.into();
    let response_json = response.to_json_string(output_format.is_pretty())?;

    print!("{response_json}");

    Ok(())
}

async fn handle_announce(
    remote_addr: SocketAddr,
    info_hash: &TorrustInfoHash,
    params: &AnnounceParams,
) -> Result<Response, Error> {
    let transaction_id = TransactionId::new(RANDOM_TRANSACTION_ID);

    let client = checker::Client::new(remote_addr, DEFAULT_TIMEOUT).await?;

    let connection_id = client.send_connection_request(transaction_id).await?;

    client
        .send_announce_request(transaction_id, connection_id, *info_hash, params)
        .await
}

async fn handle_scrape(remote_addr: SocketAddr, info_hashes: &[TorrustInfoHash]) -> Result<Response, Error> {
    let transaction_id = TransactionId::new(RANDOM_TRANSACTION_ID);

    let client = checker::Client::new(remote_addr, DEFAULT_TIMEOUT).await?;

    let connection_id = client.send_connection_request(transaction_id).await?;

    client.send_scrape_request(connection_id, transaction_id, info_hashes).await
}

fn parse_socket_addr(tracker_socket_addr_str: &str) -> anyhow::Result<SocketAddr> {
    tracing::debug!("Tracker socket address: {tracker_socket_addr_str:#?}");

    let resolved_addr = if let Ok(url) = Url::parse(tracker_socket_addr_str) {
        tracing::debug!("Tracker socket address URL: {url:?}");

        let host = url
            .host_str()
            .with_context(|| format!("invalid host in URL: `{tracker_socket_addr_str}`"))?
            .to_owned();

        let port = url
            .port()
            .with_context(|| format!("port not found in URL: `{tracker_socket_addr_str}`"))?
            .to_owned();

        (host, port)
    } else {
        let parts: Vec<&str> = tracker_socket_addr_str.split(':').collect();

        if parts.len() != 2 {
            return Err(anyhow::anyhow!(
                "invalid address format: `{tracker_socket_addr_str}`. Expected format is host:port"
            ));
        }

        let host = parts[0].to_owned();

        let port = parts[1]
            .parse::<u16>()
            .with_context(|| format!("invalid port: `{}`", parts[1]))?
            .to_owned();

        (host, port)
    };

    tracing::debug!("Resolved address: {resolved_addr:#?}");

    let socket_addrs: Vec<_> = resolved_addr.to_socket_addrs()?.collect();
    if socket_addrs.is_empty() {
        Err(anyhow::anyhow!("DNS resolution failed for `{tracker_socket_addr_str}`"))
    } else {
        Ok(socket_addrs[0])
    }
}

fn parse_info_hash(info_hash_str: &str) -> anyhow::Result<TorrustInfoHash> {
    TorrustInfoHash::from_str(info_hash_str)
        .map_err(|e| anyhow::Error::msg(format!("failed to parse info-hash `{info_hash_str}`: {e:?}")))
}

fn parse_peer_id(peer_id_str: &str) -> anyhow::Result<[u8; 20]> {
    let bytes = peer_id_str.as_bytes();
    if bytes.len() != 20 {
        return Err(anyhow::anyhow!(
            "peer-id must be exactly 20 bytes, got {} bytes for `{peer_id_str}`",
            bytes.len()
        ));
    }
    let mut arr = [0_u8; 20];
    arr.copy_from_slice(bytes);

    Ok(arr)
}

fn parse_non_zero_port(port_str: &str) -> anyhow::Result<u16> {
    let port = u16::from_str(port_str).with_context(|| format!("invalid port value: `{port_str}`"))?;

    if port == 0 {
        anyhow::bail!("port must be greater than zero")
    }

    Ok(port)
}
