use std::fmt::Debug;

use super::peer_list::PeerList;

/// A data structure containing all the information about a torrent in the tracker.
///
/// This is the tracker entry for a given torrent and contains the swarm data,
/// that's the list of all the peers trying to download the same torrent.
/// The tracker keeps one entry like this for every torrent.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Torrent {
    /// A network of peers that are all trying to download the torrent associated to this entry
    pub(crate) swarm: PeerList,

    /// The number of peers that have ever completed downloading the torrent associated to this entry
    pub(crate) downloaded: u32,
}
