pub mod announce;
pub mod banning;
pub mod connect;
pub mod scrape;

#[cfg(test)]
pub(crate) mod tests {

    use futures::future::BoxFuture;
    use mockall::mock;
    use torrust_tracker_events::sender::SendError;

    use crate::event::Event;

    mock! {
        pub(crate) UdpCoreStatsEventSender {}
        impl torrust_tracker_events::sender::Sender for UdpCoreStatsEventSender {
            type Event = Event;

            fn send(&self, event: Event) -> BoxFuture<'static,Option<Result<usize,SendError<Event> > > > ;
        }
    }
}
