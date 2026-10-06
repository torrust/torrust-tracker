//! A fake HTTP tracker that answers scrape requests (BEP 48), one per connection.
use std::collections::BTreeSet;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

use torrust_tracker_http_protocol::percent_encoding::percent_decode_info_hash;
use torrust_tracker_http_protocol::v1::query::Query;

use super::{POLL_INTERVAL, ServerThread};

/// Bounds the wait for a connected client's request, so a silent peer fails
/// the test instead of blocking the fake's stop-and-join.
const REQUEST_READ_TIMEOUT: Duration = Duration::from_secs(5);

pub struct FakeHttpTracker {
    address: SocketAddr,
    _server: ServerThread,
}

impl FakeHttpTracker {
    /// Starts a fake tracker that answers every scrape with a file per requested info hash.
    pub fn start() -> Self {
        Self::start_with_scrape_limit(usize::MAX)
    }

    /// Like [`Self::start`], but scrapes get files for only the first
    /// `max_info_hashes` `info_hash` params.
    pub fn start_with_scrape_limit(max_info_hashes: usize) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").expect("fake HTTP tracker should bind");
        listener
            .set_nonblocking(true)
            .expect("fake HTTP tracker should poll for connections");
        let address = listener.local_addr().expect("fake HTTP tracker should have an address");

        let server = ServerThread::spawn(move |stop| serve(&listener, max_info_hashes, stop));

        Self {
            address,
            _server: server,
        }
    }

    pub fn url(&self) -> String {
        format!("http://{}", self.address) // DevSkim: ignore DS137138
    }
}

fn serve(listener: &TcpListener, max_info_hashes: usize, stop: &AtomicBool) {
    while !stop.load(Ordering::Relaxed) {
        match listener.accept() {
            Ok((stream, _)) => answer_scrape(&stream, max_info_hashes),
            Err(error) if error.kind() == ErrorKind::WouldBlock => thread::sleep(POLL_INTERVAL),
            Err(error) => panic!("fake HTTP tracker failed to accept: {error}"),
        }
    }
}

fn answer_scrape(stream: &TcpStream, max_info_hashes: usize) {
    stream
        .set_nonblocking(false)
        .expect("fake HTTP tracker should read the request blocking");
    stream
        .set_read_timeout(Some(REQUEST_READ_TIMEOUT))
        .expect("fake HTTP tracker should bound the request read");

    let mut reader = BufReader::new(stream);
    let mut request_line = String::new();
    reader
        .read_line(&mut request_line)
        .expect("fake HTTP tracker should read the request line");
    skip_headers(&mut reader);

    let body = bencoded_scrape_response(&requested_info_hashes(&request_line, max_info_hashes));

    let mut writer = stream;
    write!(
        writer,
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    )
    .and_then(|()| writer.write_all(&body))
    .expect("fake HTTP tracker should send its response");
}

fn skip_headers(reader: &mut impl BufRead) {
    loop {
        let mut line = String::new();
        let read = reader.read_line(&mut line).expect("fake HTTP tracker should read headers");

        if read == 0 || line == "\r\n" {
            return;
        }
    }
}

/// The first `max_info_hashes` `info_hash` params of `GET /scrape?... HTTP/1.1`, as sent.
fn requested_info_hashes(request_line: &str, max_info_hashes: usize) -> Vec<[u8; 20]> {
    let target = request_line.split_whitespace().nth(1).expect("request should have a target");
    let (_, raw_query) = target.split_once('?').expect("scrape request should have a query");
    let query = raw_query.parse::<Query>().expect("scrape query should parse");

    query
        .get_param_vec("info_hash")
        .unwrap_or_default()
        .iter()
        .take(max_info_hashes)
        .map(|raw| percent_decode_info_hash(raw).expect("info_hash should decode").bytes())
        .collect()
}

/// A bencoded response with zeroed stats. `files` is a dictionary, so repeated
/// info hashes collapse into one file, as in a real tracker.
fn bencoded_scrape_response(info_hashes: &[[u8; 20]]) -> Vec<u8> {
    let files: BTreeSet<&[u8; 20]> = info_hashes.iter().collect();

    let mut body = b"d5:filesd".to_vec();
    for info_hash in files {
        body.extend_from_slice(b"20:");
        body.extend_from_slice(info_hash);
        body.extend_from_slice(b"d8:completei0e10:downloadedi0e10:incompletei0ee");
    }
    body.extend_from_slice(b"ee");

    body
}
