use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Amount of benchmark worker threads
    #[arg(short, long)]
    pub threads: usize,
    /// Amount of time in ns a thread will sleep to simulate a client response after handling a task
    #[arg(short, long)]
    pub sleep: Option<u64>,
    /// Compare with old implementations of the torrent repository
    #[arg(short, long)]
    pub compare: Option<bool>,
}
