pub mod announce;
pub mod scrape;

use derive_more::{Display, From, Into};

#[derive(Debug, Display, From, Into, PartialEq, Eq, Clone, Copy)]
#[display(fmt = "{query}")]
pub struct Announce {
    query: announce::Query,
}

#[derive(Debug, Display, From, Into, PartialEq, Eq, Clone)]
#[display(fmt = "{query}")]
pub struct Scrape {
    query: scrape::Query,
}
