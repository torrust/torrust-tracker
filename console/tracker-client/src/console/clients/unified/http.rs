use std::net::IpAddr;
use std::str::FromStr;
use std::time::Duration;

use anyhow::{Context, bail};
use bencode2json::try_bencode_to_json;
use clap::{Subcommand, ValueEnum};
use reqwest::Url;
use torrust_info_hash::InfoHash;
use torrust_peer_id::PeerId;
use torrust_tracker_client::http::client::Client;
use torrust_tracker_http_protocol::v1::requests::announce::{AnnounceBuilder, Compact, Event};
use torrust_tracker_http_protocol::v1::requests::scrape_builder;
use torrust_tracker_http_protocol::v1::responses::announce_deserialization::{Announce, DeserializedCompact};
use torrust_tracker_http_protocol::v1::responses::scrape_deserialization;

use super::app::OutputFormat;
use crate::DEFAULT_NETWORK_TIMEOUT;

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum CliEvent {
    Started,
    Stopped,
    Completed,
}

impl From<CliEvent> for Event {
    fn from(value: CliEvent) -> Self {
        match value {
            CliEvent::Started => Self::Started,
            CliEvent::Stopped => Self::Stopped,
            CliEvent::Completed => Self::Completed,
        }
    }
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum CliCompact {
    #[value(name = "0")]
    NotAccepted,
    #[value(name = "1")]
    Accepted,
}

impl From<CliCompact> for Compact {
    fn from(value: CliCompact) -> Self {
        match value {
            CliCompact::NotAccepted => Self::NotAccepted,
            CliCompact::Accepted => Self::Accepted,
        }
    }
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Announce {
        tracker_url: String,
        info_hash: String,
        #[arg(long)]
        event: Option<CliEvent>,
        #[arg(long)]
        uploaded: Option<u64>,
        #[arg(long)]
        downloaded: Option<u64>,
        #[arg(long)]
        left: Option<u64>,
        #[arg(long, value_parser = parse_non_zero_port)]
        port: Option<u16>,
        #[arg(long = "peer-addr")]
        peer_addr: Option<IpAddr>,
        #[arg(long = "peer-id", value_parser = parse_peer_id)]
        peer_id: Option<PeerId>,
        #[arg(long, value_enum)]
        compact: Option<CliCompact>,
        #[arg(long, value_enum, default_value_t = OutputFormat::Json)]
        format: OutputFormat,
    },
    Scrape {
        tracker_url: String,
        info_hashes: Vec<String>,
        #[arg(long, value_enum, default_value_t = OutputFormat::Json)]
        format: OutputFormat,
    },
}

struct AnnounceOptions {
    tracker_url: String,
    info_hash: String,
    event: Option<CliEvent>,
    uploaded: Option<u64>,
    downloaded: Option<u64>,
    left: Option<u64>,
    port: Option<u16>,
    peer_addr: Option<IpAddr>,
    peer_id: Option<PeerId>,
    compact: Option<CliCompact>,
    output_format: OutputFormat,
}

/// # Errors
///
/// Returns an error if the command fails.
pub async fn run(command: Command) -> anyhow::Result<()> {
    match command {
        Command::Announce {
            tracker_url,
            info_hash,
            event,
            uploaded,
            downloaded,
            left,
            port,
            peer_addr,
            peer_id,
            compact,
            format,
        } => {
            announce_command(
                AnnounceOptions {
                    tracker_url,
                    info_hash,
                    event,
                    uploaded,
                    downloaded,
                    left,
                    port,
                    peer_addr,
                    peer_id,
                    compact,
                    output_format: format,
                },
                DEFAULT_NETWORK_TIMEOUT,
            )
            .await?;
        }
        Command::Scrape {
            tracker_url,
            info_hashes,
            format,
        } => {
            scrape_command(&tracker_url, &info_hashes, format, DEFAULT_NETWORK_TIMEOUT).await?;
        }
    }

    Ok(())
}

async fn announce_command(options: AnnounceOptions, timeout: Duration) -> anyhow::Result<()> {
    let base_url = parse_and_validate_tracker_url(&options.tracker_url)?;
    let info_hash = InfoHash::from_str(&options.info_hash).map_err(|_| {
        anyhow::anyhow!(
            "invalid infohash `{}`. Example infohash: `9c38422213e30bff212b30c360d26f9a02136422`",
            options.info_hash
        )
    })?;

    let mut query_builder = AnnounceBuilder::with_default_values().with_info_hash(&info_hash);

    if let Some(event) = options.event {
        query_builder = query_builder.with_event(event.into());
    }
    if let Some(uploaded) = options.uploaded {
        query_builder = query_builder.with_uploaded(uploaded);
    }
    if let Some(downloaded) = options.downloaded {
        query_builder = query_builder.with_downloaded(downloaded);
    }
    if let Some(left) = options.left {
        query_builder = query_builder.with_left(left);
    }
    if let Some(port) = options.port {
        query_builder = query_builder.with_port(port);
    }
    if let Some(peer_addr) = options.peer_addr {
        query_builder = query_builder.with_peer_addr(peer_addr);
    }
    if let Some(peer_id) = options.peer_id {
        query_builder = query_builder.with_peer_id(&peer_id);
    }
    if let Some(compact) = options.compact {
        query_builder = query_builder.with_compact(compact.into());
    }

    let response = Client::new(base_url, timeout)?.announce(&query_builder.query()).await?;

    let body = response.bytes().await?;

    let json = if let Ok(announce_response) = serde_bencode::from_bytes::<Announce>(&body) {
        serialize_json(&announce_response, options.output_format).context("failed to serialize announce response into JSON")?
    } else if let Ok(compact_response) = serde_bencode::from_bytes::<DeserializedCompact>(&body) {
        serialize_json(&compact_response, options.output_format)
            .context("failed to serialize compact announce response into JSON")?
    } else {
        let fallback = bencode_to_fallback_json_or_raw_bytes(&body, options.output_format)
            .context("failed to serialize fallback announce response into JSON")?;

        println!("{fallback}");

        bail!("unrecognized announce response from tracker")
    };

    println!("{json}");

    Ok(())
}

async fn scrape_command(
    tracker_url: &str,
    info_hashes: &[String],
    output_format: OutputFormat,
    timeout: Duration,
) -> anyhow::Result<()> {
    let base_url = parse_and_validate_tracker_url(tracker_url)?;

    let query = scrape_builder::Query::try_from(info_hashes).context("failed to parse infohashes")?;

    let response = Client::new(base_url, timeout)?.scrape(&query).await?;

    let body = response.bytes().await?;

    let Ok(scrape_response) = scrape_deserialization::Response::try_from_bencoded(&body) else {
        let fallback = bencode_to_fallback_json_or_raw_bytes(&body, output_format)
            .context("failed to serialize fallback scrape response into JSON")?;

        println!("{fallback}");

        bail!("unrecognized scrape response from tracker")
    };

    let json = serialize_json(&scrape_response, output_format).context("failed to serialize scrape response into JSON")?;

    println!("{json}");

    Ok(())
}

fn parse_peer_id(peer_id_str: &str) -> anyhow::Result<PeerId> {
    let bytes = peer_id_str.as_bytes();
    if bytes.len() != 20 {
        return Err(anyhow::anyhow!(
            "peer-id must be exactly 20 bytes, got {} bytes for `{peer_id_str}`",
            bytes.len()
        ));
    }

    let mut arr = [0_u8; 20];
    arr.copy_from_slice(bytes);

    Ok(PeerId(arr))
}

fn parse_non_zero_port(port_str: &str) -> anyhow::Result<u16> {
    let port = u16::from_str(port_str).with_context(|| format!("invalid port value: `{port_str}`"))?;

    if port == 0 {
        anyhow::bail!("port must be greater than zero")
    }

    Ok(port)
}

fn parse_and_validate_tracker_url(tracker_url: &str) -> anyhow::Result<Url> {
    let url = Url::parse(tracker_url).context("failed to parse HTTP tracker base URL")?;

    validate_tracker_url_parts(&url)?;

    Ok(url)
}

fn validate_tracker_url_parts(url: &Url) -> anyhow::Result<()> {
    if url.query().is_some() || url.fragment().is_some() {
        bail!(
            "invalid tracker URL input: include only scheme, host, optional port, and optional path. Do not include query or fragment. Pass tracker request params using dedicated CLI arguments"
        );
    }

    Ok(())
}

fn bencode_to_fallback_json_or_raw_bytes(body: &[u8], output_format: OutputFormat) -> anyhow::Result<String> {
    match try_bencode_to_json(body) {
        Ok(json) => match output_format {
            OutputFormat::Json => Ok(json),
            OutputFormat::Text => {
                let value: serde_json::Value = serde_json::from_str(&json).context("failed to parse fallback bencode JSON")?;

                serialize_json(&value, output_format).context("failed to format fallback bencode JSON")
            }
        },
        Err(_) => Ok(format!(
            "Warning: Could not deserialize HTTP tracker response. Raw bytes: {body:?}"
        )),
    }
}

fn serialize_json<T: serde::Serialize>(value: &T, output_format: OutputFormat) -> anyhow::Result<String> {
    if output_format.is_pretty() {
        serde_json::to_string_pretty(value).context("failed to serialize pretty JSON")
    } else {
        serde_json::to_string(value).context("failed to serialize JSON")
    }
}

#[cfg(test)]
mod tests {
    use reqwest::Url;
    use serde::Serialize;

    use super::{parse_and_validate_tracker_url, serialize_json, validate_tracker_url_parts};
    use crate::console::clients::unified::app::OutputFormat;

    #[derive(Serialize)]
    struct Sample {
        seeders: i32,
        leechers: i32,
    }

    #[test]
    fn it_should_serialize_json_output() {
        let data = Sample { seeders: 1, leechers: 2 };

        let json = serialize_json(&data, OutputFormat::Json).expect("it should serialize compact JSON");

        assert_eq!(json, "{\"seeders\":1,\"leechers\":2}");
    }

    #[test]
    fn it_should_serialize_text_output_as_pretty_json() {
        let data = Sample { seeders: 1, leechers: 2 };

        let json = serialize_json(&data, OutputFormat::Text).expect("it should serialize pretty JSON");

        assert!(json.contains('\n'));
        assert!(json.contains("  \"seeders\": 1"));
        assert!(json.contains("  \"leechers\": 2"));
    }

    #[test]
    fn it_accepts_tracker_url_with_path_and_without_query_or_fragment() {
        let parsed = parse_and_validate_tracker_url("https://tracker.example.com/announce");

        assert!(parsed.is_ok());
    }

    #[test]
    fn it_rejects_tracker_url_with_query() {
        let parsed = parse_and_validate_tracker_url("https://tracker.example.com/announce?info_hash=abc");

        assert!(parsed.is_err());
    }

    #[test]
    fn it_rejects_tracker_url_with_fragment() {
        let parsed = parse_and_validate_tracker_url("https://tracker.example.com/announce#details");

        assert!(parsed.is_err());
    }

    #[test]
    fn it_accepts_direct_validation_for_plain_base_url() {
        let url = Url::parse("https://tracker.example.com/").expect("url should parse");

        let result = validate_tracker_url_parts(&url);

        assert!(result.is_ok());
    }
}
