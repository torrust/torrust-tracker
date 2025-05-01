pub mod peer_list;
pub mod torrent;

use std::sync::{Arc, Mutex};

pub type TorrentEntry = Arc<Mutex<torrent::Torrent>>;
