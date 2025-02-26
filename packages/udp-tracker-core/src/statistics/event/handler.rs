use crate::statistics::event::Event;
use crate::statistics::repository::Repository;

pub async fn handle_event(event: Event, stats_repository: &Repository) {
    match event {
        // UDP4
        Event::Udp4Connect => {
            stats_repository.increase_udp4_connections().await;
        }
        Event::Udp4Announce => {
            stats_repository.increase_udp4_announces().await;
        }
        Event::Udp4Scrape => {
            stats_repository.increase_udp4_scrapes().await;
        }

        // UDP6
        Event::Udp6Connect => {
            stats_repository.increase_udp6_connections().await;
        }
        Event::Udp6Announce => {
            stats_repository.increase_udp6_announces().await;
        }
        Event::Udp6Scrape => {
            stats_repository.increase_udp6_scrapes().await;
        }
    }

    tracing::debug!("stats: {:?}", stats_repository.get_stats().await);
}

#[cfg(test)]
mod tests {
    use crate::statistics::event::handler::handle_event;
    use crate::statistics::event::Event;
    use crate::statistics::repository::Repository;

    #[tokio::test]
    async fn should_increase_the_udp4_connections_counter_when_it_receives_a_udp4_connect_event() {
        let stats_repository = Repository::new();

        handle_event(Event::Udp4Connect, &stats_repository).await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.udp4_connections_handled, 1);
    }

    #[tokio::test]
    async fn should_increase_the_udp4_announces_counter_when_it_receives_a_udp4_announce_event() {
        let stats_repository = Repository::new();

        handle_event(Event::Udp4Announce, &stats_repository).await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.udp4_announces_handled, 1);
    }

    #[tokio::test]
    async fn should_increase_the_udp4_scrapes_counter_when_it_receives_a_udp4_scrape_event() {
        let stats_repository = Repository::new();

        handle_event(Event::Udp4Scrape, &stats_repository).await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.udp4_scrapes_handled, 1);
    }

    #[tokio::test]
    async fn should_increase_the_udp6_connections_counter_when_it_receives_a_udp6_connect_event() {
        let stats_repository = Repository::new();

        handle_event(Event::Udp6Connect, &stats_repository).await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.udp6_connections_handled, 1);
    }

    #[tokio::test]
    async fn should_increase_the_udp6_announces_counter_when_it_receives_a_udp6_announce_event() {
        let stats_repository = Repository::new();

        handle_event(Event::Udp6Announce, &stats_repository).await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.udp6_announces_handled, 1);
    }

    #[tokio::test]
    async fn should_increase_the_udp6_scrapes_counter_when_it_receives_a_udp6_scrape_event() {
        let stats_repository = Repository::new();

        handle_event(Event::Udp6Scrape, &stats_repository).await;

        let stats = stats_repository.get_stats().await;

        assert_eq!(stats.udp6_scrapes_handled, 1);
    }
}
