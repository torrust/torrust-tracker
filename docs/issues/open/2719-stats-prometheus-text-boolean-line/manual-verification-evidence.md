# Manual Verification Evidence

Recorded 2026-11-04 (UTC) by Claude Code (Claude Sonnet 5.5) at `torrust/develop` commit `08efcf763`, Rust dev profile build of
`torrust-tracker`. The development token `MyAccessToken` is public and documented.

## Prerequisites

- Persistence off: `share/default/config/tracker.container.no-persistence.toml` plus an appended `[http_api]` section (port 1212,
  token `MyAccessToken`); the file has no REST API section of its own.
- Persistence on: `share/default/config/tracker.development.sqlite3.toml` (`persistent_torrent_completed_stat = true`) with the
  SQLite path moved to a scratch directory.
- Tools: `curl`, Prometheus and `promtool` 3.5.0 (`prom/prometheus:v3.5.0` container, image digest
  `sha256:63805ebb8d2b3920190daf1cb14a60871b16fd38bed42b857a3182bc621f4996`), Python 3.14 with `prometheus_client` 0.26.0.

## V1 - Raw output, persistence off and on

```bash
cargo build --bin torrust-tracker
./target/debug/torrust-tracker --config-toml-path off.toml &
curl -si 'http://localhost:1212/api/v1/stats?format=prometheus&token=MyAccessToken'
```

```text
HTTP/1.1 200 OK
content-type: text/plain; charset=utf-8
content-length: 736

torrents 0
seeders 0
completed 0
completed_in_session 0
completed_persisted 0
completed_persisted_enabled false
leechers 0
...
udp6_errors_handled 0
```

With the persistence-on configuration the same line reads `completed_persisted_enabled true`. The last byte of the body is `0`
(`0x30`), not `0x0a`:

```text
$ tail -c 1 stats-on.txt | xxd
00000000: 30                                       0
```

Without `format` the endpoint answers JSON (`"completed_persisted_enabled":true`). `GET /api/v1/metrics?format=prometheus`
answers real exposition text with `# HELP`/`# TYPE` lines and numeric values only, ending with a newline.

## V2 - Parser checks on the saved body

`promtool check metrics < body` (3.5.0):

| Body                                      | Result                                                               |
| ----------------------------------------- | -------------------------------------------------------------------- |
| as served, persistence on                 | `text format parsing error in line 6: expected float as value, got "true"` (exit 1)  |
| as served, persistence off                | `... got "false"` (exit 1)                                           |
| `true` replaced by `1`, no final newline  | `text format parsing error in line 32: unexpected end of input stream` (exit 1)      |
| line dropped, no final newline            | `text format parsing error in line 31: unexpected end of input stream` (exit 1)      |
| line dropped, final newline               | lint warnings only (`... no help text`), exit 3                      |
| `true` kept, final newline                | `... got "true"` (exit 1)                                            |

`prometheus_client.parser.text_string_to_metric_families` 0.26.0:

| Body                                      | Result                                                  |
| ----------------------------------------- | ------------------------------------------------------- |
| as served (`true` / `false`)              | `ValueError: could not convert string to float: 'true'` / `'false'` |
| `true` replaced by `1`, no final newline  | 32 families parsed (the missing newline is tolerated)   |
| line dropped, no final newline            | 31 families parsed                                      |

Verdict per defect: the **boolean value fails** both tools. The **missing trailing newline fails** `promtool` only; the Python
parser and the Prometheus server accept it (V3). The **absence of `# TYPE`** is allowed and produces only lint warnings.

## V3 - Real Prometheus 3.5.0 scraping

Prometheus scraped the running tracker (persistence on) at a 1 s interval with
`params: {format: [prometheus], token: [MyAccessToken]}`, and static copies of the bodies served by `python3 -m http.server`
(with `fallback_scrape_protocol: PrometheusText0.0.4`, because a static server sends `application/octet-stream`):

| Target                                           | Health | Detail                                                                     | `scrape_samples_scraped` |
| ------------------------------------------------ | ------ | -------------------------------------------------------------------------- | ------------------------ |
| tracker `/api/v1/stats?format=prometheus`        | down   | `strconv.ParseFloat: parsing "true": invalid syntax while parsing: "completed_persisted_enabled true"` | 5 |
| tracker `/api/v1/metrics?format=prometheus`      | up     |                                                                            | 3                        |
| static, as served, persistence on                | down   | same error with `"true"`                                                   | 5                        |
| static, as served, persistence off               | down   | same error with `"false"`                                                  | 5                        |
| static, `true` replaced by `1`, no final newline | up     |                                                                            | 32                       |
| static, line dropped, no final newline           | up     |                                                                            | 31                       |
| static, line dropped, final newline              | up     |                                                                            | 31                       |

The tracker's `text/plain; charset=utf-8` content type is accepted by Prometheus 3.5.0. The legacy text is scraped up to the
boolean line; the 26 samples after it are lost and the target is `down`.

## V4 - Regression test red and green

Stable Rust toolchain, `cargo test -p torrust-tracker-axum-rest-api-server --lib stats::responses`.

Red, at commit `test(axum-rest-api-server): [#2719] expect numeric values and a final newline in the legacy stats text` on top of the
unfixed `metrics_response`: `it_should_answer_the_statistics_as_one_name_and_value_line_per_statistic` and
`it_should_print_the_persistence_flag_as_zero_when_persisted_completed_statistics_are_disabled` fail (`3 passed; 2 failed`). The
first reports the body `...completed_persisted_enabled true\n...udp6_errors_handled 32` against the expected
`...completed_persisted_enabled 1\n...udp6_errors_handled 32\n`.

Green, at `fix(axum-rest-api-server): [#2719] print numeric values and a final newline in the legacy stats text`: the whole package
passes (`157 passed` unit, `58 passed` integration, `0 failed`).

## V5 - Like-for-like recheck after the fix

Build of the final production code (`cargo build --bin torrust-tracker`, branch head `501a5cc66`; the later commits only touch
documentation and this evidence). Same configurations, same commands as V1 to V3.

```bash
curl -si 'http://localhost:1212/api/v1/stats?format=prometheus&token=MyAccessToken'
```

```text
HTTP/1.1 200 OK
content-type: text/plain; charset=utf-8
content-length: 733

torrents 0
...
```

The line is `completed_persisted_enabled 0` with persistence off and `completed_persisted_enabled 1` with it on; the last byte of the body is
`0x0a`.

| Check                                            | Result                                                        |
| ------------------------------------------------ | ------------------------------------------------------------- |
| `promtool check metrics` 3.5.0, off and on       | only `no help text` lint warnings, exit 3 (parse succeeds)    |
| `prometheus_client` 0.26.0, off and on           | 32 families parsed                                            |
| Prometheus 3.5.0 scraping the tracker, persistence on | target `up`, `scrape_samples_scraped` 32 (was `down`, 5)  |
| Prometheus 3.5.0 scraping `/api/v1/metrics?format=prometheus` | target `up`                                     |
