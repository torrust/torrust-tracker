---
doc-type: issue-lessons
issue: 1348
package: torrust-tracker-udp-core
---

# UDP Core Testing Lessons

Candidate lessons collected while working on Issue #1348. Promoting any of them into
`docs/testing/`, `.github/skills/`, or a production change is a separate decision after this
issue, per the [subissue guidelines](../1347-overhaul-packages-testing/subissue-spec-guidelines.md).

## Follow-Up Candidates

| Source | Candidate | Evidence |
| --- | --- | --- |
| [Scrape plan](test-refactor-plans/services-scrape-tests.md) | Restore the reserved-variant rationale in the `ScrapeHandler::handle_scrape` docs: `ScrapeError::Whitelist` exists so a future non-zeroing whitelist policy does not break callers. | The #1270 refactor (`da1353bf1`) stated the intent; the current doc only mentions the persisted-downloads failure. |
| [Scrape plan](test-refactor-plans/services-scrape-tests.md) | Decide whether to remove `UdpScrapeError::TrackerCoreWhitelistError` and its `From<WhitelistError>`. | The scrape service never authorizes; a tracker-core whitelist failure would arrive as `TrackerCoreScrapeError`. |
| [Cookie plan](test-refactor-plans/connection-cookie-tests.md) | Change the `ValueExpired`/`ValueFromFuture` messages from `expected >`/`expected <` to `expected >=`/`expected <=`. | `check` accepts both range ends; two tests pin that behavior. |
| [Cookie plan](test-refactor-plans/connection-cookie-tests.md) | Correct the module docs that say special values are excluded by `is_finite()`; the code uses `is_normal()`, which also excludes zero and subnormal values. | `connection_cookie.rs` module docs versus `make`/`check`. |
| [Keys plan](test-refactor-plans/crypto-keys-tests.md) | **Taken up as Issue #2458** ([spec](../../closed/2458-inject-udp-cookie-cipher/ISSUE.md)): inject one cookie key. The startup `check_seed()` compares an unused seed, so it cannot detect production encrypting cookies with the public all-zero test key, which is compiled into and initialized in production. | The cookie cipher has its own random key, not derived from `RANDOM_SEED`, since commit `e3562f069`. |
| [Banning plan](test-refactor-plans/services-banning-tests.md) | **Handed off as a bug (issue pending).** `get_banned_ips_total()` counts every IP with an invalid connection ID, not the banned ones, and the gauge `udp_tracker_server_ips_banned_total` (REST `udp_banned_ips_total`) goes stale after the periodic reset. | `udp-server`'s own handler test already calls it "the distinct client IP total". |
| [Banning plan](test-refactor-plans/services-banning-tests.md) | Remove `BanService::last_connection_id_errors_reset` (included in the banning bug draft). | Written by the constructor and `reset_bans()`; read nowhere in the repository. |
| [Event plan](test-refactor-plans/event-tests.md) | Remove `ConnectionContext::server_socket_addr()`. | No caller anywhere in the repository; the label projection reads the binding directly. |
| [Metrics plan](test-refactor-plans/statistics-metrics-tests.md) | Remove `udp-core`'s `Metrics::set_gauge`. | No caller anywhere in the repository. |
| [Metrics plan](test-refactor-plans/statistics-metrics-tests.md) | Correct the "from IPv4/IPv6 peers" docs of the `udp4_*`/`udp6_*` counters in `udp-core`, `udp-server`, and the REST API stats resource: they count by the server socket's address family, so on a dual-stack `[::]` listener IPv4 clients count as `udp6`. | Every projection filters on `server_binding_address_ip_family`, not the client's family. |
| [Repository plan](test-refactor-plans/statistics-repository-tests.md) | **Handed off as an EPIC #1669 sub-issue draft (issue pending).** Remove the unused SI-30 traits `BanningStats`, `UdpCoreStatsRepository`, and `UdpServerStatsRepository`. | No consumer in the repository; the REST API decouples through `rest-api-application`'s `StatsQueryPort`, implemented over the concrete repositories. |
| [Repository plan](test-refactor-plans/statistics-repository-tests.md) | **Handed off as an EPIC #1669 sub-issue draft (issue pending).** Make the `pub stats` lock private in the five statistics repositories. | No caller outside each repository file reads or writes the field. |
| [Services plan](test-refactor-plans/statistics-services-tests.md) | **Forwarded by the maintainer to the unused SI-30 traits draft.** Remove `statistics::services::get_metrics()` and `TrackerMetrics` from `udp-core` and `udp-server`. | No consumer outside their own tests; the REST runtime adapter reads the statistics repositories directly. |

## Testing Lessons

- **A failing collaborator store is cheaper than it looks.** `tracker-core` exports
  `MockTorrentMetricsStore` unconditionally, so a `udp-core` unit test can reach database-failure
  adaptation paths without a database. Assumed-unreachable error paths should be re-checked
  against the collaborator's exported test doubles before they are recorded as untestable.
- **An expectation-free mock proves an ordering decision.** Giving the service a store with no
  expectations and an invalid cookie proves the cookie is checked before any database read: the
  mock panics on the first call. A temporary order swap confirmed the test fails.
- **A property test earned its dependency by catching a mutation the examples missed.** Replacing
  the cookie's wrapping arithmetic with saturating arithmetic passed all 13 example tests in
  `connection_cookie.rs` (they use a small fingerprint) and failed only the round-trip property.
  The ban-threshold property in `services/banning.rs` repeated this: a truncating `as u8` comparison
  passed all six examples (limit 1) and failed only the property. Examples that fix one parameter
  value cannot catch defects that depend on that parameter.
- **Tests that pin a mechanism let its purpose drift.** The `crypto/keys.rs` tests assert that test
  builds alias the zeroed key, but nothing asserted that production cookies use a secret key. When
  the design moved from the seed to a separate cipher, the tests and the startup check kept
  passing while guarding the wrong value. Test the security property, not the wiring trick.
