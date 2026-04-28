#[derive(Clone, Copy, Debug)]
pub(super) enum ClientRole {
    Seeder,
    Leecher,
}

impl ClientRole {
    pub(super) const fn service_name(self) -> &'static str {
        match self {
            Self::Seeder => "qbittorrent-seeder",
            Self::Leecher => "qbittorrent-leecher",
        }
    }

    pub(super) const fn client_label(self) -> &'static str {
        match self {
            Self::Seeder => "seeder",
            Self::Leecher => "leecher",
        }
    }
}
