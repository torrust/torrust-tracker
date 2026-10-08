---
semantic-links:
  related-artifacts:
    - "issue #2491"
    - "issue #1659"
    - "issue #144"
    - "issue #2411"
    - docs/issues/open/2491-i2p-peer-support/EPIC.md
    - docs/issues/closed/1987-add-config-option-to-use-ip-from-announce-query-string/ISSUE.md
    - docs/discussions/2491-i2p-peer-support/20261008-i2p-peer-support-design/review-pass-1.md
    - docs/discussions/2491-i2p-peer-support/20261008-i2p-peer-support-design/destination-spoofing-analysis.md
    - docs/discussions/2491-i2p-peer-support/20261008-i2p-peer-support-design/i2p-addressing-primer.md
    - docs/discussions/2491-i2p-peer-support/20261008-i2p-peer-support-design/manual-test-evidence.md
    - docs/discussions/AGENTS.md
    - docs/templates/DISCUSSION.md
---

<!-- cspell:ignore Erdosi Szucs -->

# I2P Peer Support Design

| Field          | Value                                                                                                                                                                                                      |
| -------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Status         | Open for rounds                                                                                                                                                                                            |
| Started        | 2026-10-08                                                                                                                                                                                                 |
| Opened by      | Jose Celano (`josecelano`)                                                                                                                                                                                 |
| Decision owner | Not assigned yet: the owner of EPIC #2491, once assigned                                                                                                                                                   |
| Informs        | EPIC #2491 - I2P peer support                                                                                                                                                                              |
| Scope          | The design decisions needed before the tracker supports I2P peers, using PR #2050 and its review in PR #2059 as input. UDP over I2P, I2P PEX, and I2P DHT are left out; the EPIC lists them as out of scope |
| AI assistance  | AI assistant using the Copilot SDK in VS Code (model not recorded): research of both PRs and their branches, and the draft of this document                                                               |

## Context

### Why This Discussion

On 2026-07-31, Frigyes Erdosi Szucs (`Frigyes06`) opened PR #2050 to add I2P peer support to the
HTTP tracker. The maintainers, Jose Celano (`josecelano`) and Cameron (`da2ce7`), reviewed it. Jose
Celano rebased and signed it in the draft PR #2059, keeping the contributor as author of the
implementation commits, and added a review with seven required actions.

Both maintainers support the feature and think it is not hard to implement. They also think it
needs a careful design that covers its edge cases before any of it merges. The maintainers are
focused on releasing v4.0.0 (#1659), so the work is postponed until after that release, when the
roadmap is redefined.

This discussion keeps everything learned so far, so that PRs #2050 and #2059 can be closed without
losing it. Closing them does not reject the feature: they stay on GitHub as reference, and
EPIC #2491 tracks the work.

Cameron's concerns are not written in either PR. Under the round rules in
[`docs/discussions/AGENTS.md`](../../AGENTS.md), he adds them in his own round.

Nothing here changes a specification or a rule. A conclusion takes effect only when the decision
owner records it in the Outcome and in the canonical document it links.

### Timeline

| Date       | Event                                                                                                                                                                                                                        |
| ---------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 2026-07-31 | Frigyes06 opens PR #2050, "Feat/i2p peer support": 2 commits, 45 files, no description.                                                                                                                                      |
| 2026-08-17 | The Copilot reviewer comments on PR #2050 with three findings: aggregate announce statistics, malformed suffixless Destinations, and the compact response parser.                                                            |
| 2026-08-17 | Jose Celano opens draft PR #2059 with the rebased, signed commits and the review documents, and asks Cameron in [a PR #2050 comment](https://github.com/torrust/torrust-tracker/pull/2050#issuecomment-5318276034) whether the project wants the protocol. |
| 2026-08-17 | The Copilot reviewer comments on PR #2059 with eight findings and one suppressed finding; the review documents absorb them.                                                                                                  |
| 2026-08-18 | Jose Celano reports the Destination spoofing problem as a merge blocker in [a PR #2050 comment](https://github.com/torrust/torrust-tracker/pull/2050#issuecomment-5327048048).                                                 |
| 2026-09-10 | Jose Celano comments on both PRs that I2P support is delayed until v4.0.0 is released.                                                                                                                                       |
| 2026-10-08 | EPIC #2491 and this discussion are opened. The PRs are to be closed after this discussion merges.                                                                                                                           |

### Where the Material Lives

| Artifact                                                                                | What it contains                                                                                                                                       |
| --------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------ |
| [PR #2050](https://github.com/torrust/torrust-tracker/pull/2050)                        | The original proposal, branch `Frigyes06:feat/i2p-peer-support`, head `ca3006f5b`; review comments by Copilot and Jose Celano                           |
| [PR #2059](https://github.com/torrust/torrust-tracker/pull/2059)                        | The rebased draft, branch `josecelano:2050-i2p-peer-support-reviewed`, head `883ab677f`, base `ceaf16564`; its description lists the required actions |
| [`review-pass-1.md`](review-pass-1.md)                                                  | The maintainer review: architecture, findings 3.1 to 3.14, actions A1 to A7 and F1 to F5, and answers from the I2P BitTorrent specification             |
| [`destination-spoofing-analysis.md`](destination-spoofing-analysis.md)                  | The threat model for claimed Destinations, the deployment options, and the minimum policy before merge                                                 |
| [`i2p-addressing-primer.md`](i2p-addressing-primer.md)                                  | How I2P addresses peers: Destinations, hashes, `.b32.i2p` names, and the announce and response formats                                                 |
| [`manual-test-evidence.md`](manual-test-evidence.md)                                    | Reproducible `curl` checks against the PR build, with observed output                                                                                  |

The four documents are copies of `docs/pr-reviews-manual/pr-2050/` on the PR #2059 branch, kept
here as source material. Each one says at its top what changed to pass the linters. The PR branch
also edited the issue #1987 specification and added `docs/pr-reviews-manual/README.md`; those
edits are not copied, and the first is summarized below.

### What the Proposal Does

PR #2050 adds I2P support to the HTTP tracker only. In summary, from the review:

- **Peer address model.** A new `PeerAddress` enum, `Clearnet(SocketAddr)` or `I2p(...)`, replaces
  `SocketAddr` as `Peer::peer_addr`. The change touches most packages, because `Peer` stops being
  `Copy`.
- **Destination type.** A new `I2pDestination` in `packages/primitives/src/i2p.rs` decodes the I2P
  Base64 alphabet, checks the minimum length (387 bytes) and the certificate length, normalizes the
  `.i2p` suffix, and computes the SHA-256 hash once. The `primitives` package gains the `base64`
  and `sha2` dependencies.
- **Announce.** A client sends its full Destination in the `ip` query parameter. The parser tries
  an IP address first, then an I2P Destination. The query parser now splits each pair at the first
  `=` only, so Base64 padding survives.
- **Responses.** Non-compact responses carry the full Destination in `ip` and a placeholder port of
  `1`. Compact responses carry concatenated 32-byte Destination hashes, as the I2P BitTorrent
  specification requires.
- **Isolation.** The swarm is keyed by `PeerAddress`, and peer selection filters by network, so I2P
  and clearnet peers never receive each other. The UDP tracker skips I2P peers.
- **REST API.** The adapter serializes an I2P peer address with `to_string()`, so the existing
  `peer_addr` field would carry a full Destination.

### What the Review Found

The manual tests confirmed that I2P-to-I2P matchmaking and network isolation work. The review
recorded these actions; the source has the details, acceptance criteria, and tests for each.

| ID  | Action                                                                                   | Kind                    | Source                                                                                                      |
| --- | ---------------------------------------------------------------------------------------- | ----------------------- | ----------------------------------------------------------------------------------------------------------- |
| A1  | Percent-decode query values once, so `%3D%3D` padding is accepted                        | Required                | Review 3.2; PR #2059 Copilot comment on `query.rs`                                                          |
| A2  | Make compact response deserialization I2P-aware instead of reading 6-byte IPv4 chunks    | Required                | Review 3.3; Copilot comments on `encoding.rs` in both PRs                                                   |
| A3  | Keep structured I2P parse errors instead of a generic `InvalidParam`                     | Required                | Review 3.9; PR #2059 Copilot comment on `announce.rs`                                                       |
| A4  | Bound the Destination size before decoding, validate certificate types, redact errors     | Required                | Review 3.10; PR #2059 Copilot comments on `i2p.rs` and the suppressed comment                               |
| A5  | Report `complete` and `incomplete` for the requester's network only                       | Required                | Review 3.12; Copilot comments on `coordinator.rs` in both PRs                                               |
| A6  | Reject malformed suffixless Destinations instead of registering a clearnet peer          | Required                | Review 3.13; Copilot comments on `announce.rs` in both PRs                                                  |
| A7  | Do not trust a Destination from the `ip` parameter; derive it from trusted I2P transport | Required, merge blocker | Review 3.14 and the [spoofing analysis](destination-spoofing-analysis.md)                                   |
| F1  | Keep a test for the query parser's first-`=` split                                       | Follow-up               | Review 3.4                                                                                                  |
| F2  | Document why the placeholder port is `1`, not the conventional `6881`                    | Follow-up               | Review section 4                                                                                            |
| F4  | Let the tracker client send I2P Destinations and parse I2P responses                     | Follow-up               | Review 3.7                                                                                                  |
| F5  | Define the REST API representation of I2P peers in the REST API overhaul (#144)          | Follow-up               | Review 3.11; PR #2059 Copilot comment on `peer.rs`                                                          |

F3 was promoted to A5, so there is no F3.

### What Changed on `develop` Since

Checked at `develop` `23891ad8f` on 2026-10-08:

- The PR #2059 branch is 2,455 commits behind `develop`. Of the 40 code files it changes, 28 have
  changed on `develop` since its base, and a trial merge conflicts in 14 files. Reusing the code
  means reworking it, not rebasing it.
- Issue #1987 is closed. It added the `use_ip_from_query_string` HTTP tracker option (default
  `false`) without an I2P rule. The PR #2059 branch had edited the #1987 specification to add one:
  a valid I2P Destination in `ip` is always used, whatever the option says, and an ADR records that
  precedence. That edit never merged, so the precedence is open again (Q3).
- No file on `develop` mentions I2P.

## Topics

### Q1 - Scope of the first delivery

Should the first delivery cover only the HTTP tracker, as PR #2050 does, and leave UDP over I2P,
I2P PEX, and I2P DHT to later work? Should the tracker client (F4) ship with it, so that the
feature can be tested without raw `curl` commands? Added by Jose Celano.

#### Positions

No positions yet.

### Q2 - Destination trust model

A plain HTTP announce does not prove that the client owns the Destination in `ip`. An attacker can
impersonate a peer, take over its swarm record, or fill swarms with fabricated Destinations. The
[spoofing analysis](destination-spoofing-analysis.md) lists four options: reject I2P announces
until enforcement exists; accept unverified Destinations; enforce the Destination from trusted
`X-I2P-Dest*` headers added by an I2P server tunnel in front of a loopback-only listener; or a
SAMv3/I2CP adapter. Which one, and does the decision need an ADR? Added by Jose Celano.

#### Positions

##### Q2 - Jose Celano (`josecelano`), 2026-10-08, PR #[To be assigned]

Restated from my review in PR #2059 and my
[PR #2050 comment](https://github.com/torrust/torrust-tracker/pull/2050#issuecomment-5327048048).
I2P announces must not be enabled while the identity comes from the `ip` query parameter. The
first secure delivery either derives the Destination from a trusted I2P server tunnel on a
dedicated, loopback-only listener, or rejects I2P announces until that exists. An unverified
compatibility mode is not acceptable for a public tracker. The decision needs an ADR. Drafted with
an AI assistant using the Copilot SDK in VS Code from my review documents.

### Q3 - Listener configuration and `ip` precedence

Does I2P need its own listener type or per-listener mode, and how is it configured? How does a
Destination in `ip` interact with `use_ip_from_query_string` from #1987, and with
`on_reverse_proxy`? What happens to I2P identity headers on a clearnet listener? Added by Jose
Celano.

#### Positions

No positions yet.

### Q4 - Peer address model

Should `Peer::peer_addr` become a `PeerAddress` enum, as in PR #2050, or should I2P peers live in a
separate structure? The enum touches most packages and makes `Peer` non-`Copy`. It could also make
the code more generic, so that other overlay networks fit later. Added by Jose Celano.

#### Positions

##### Q4 - Jose Celano (`josecelano`), 2026-10-08, PR #[To be assigned]

Restated from my
[PR #2050 comment](https://github.com/torrust/torrust-tracker/pull/2050#issuecomment-5318276034).
I see no major issue with supporting the protocol, and I think the change could make the code a
bit more generic, so that it accepts other types of peers in the future. Drafted with an AI
assistant using the Copilot SDK in VS Code from my PR comment.

### Q5 - Swarm isolation and statistics

PR #2050 keeps one swarm per info hash and filters peers by network when it answers. Is that
enough, or should each network have its own swarm? Announce responses must count only reachable
peers (A5), but what should scrape responses, the REST API, and metrics count? Should
`max_peers_per_announce` and `numwant` apply per network? Added by Jose Celano.

#### Positions

No positions yet.

### Q6 - Wire format and input handling

These rules decide what the tracker accepts and returns: percent-decoding (A1); the rule that
identifies a suffixless Destination (A6); the maximum Destination size and the supported
certificate types (A4); structured errors that never echo the Destination (A3); the placeholder
port (F2); and compact response parsing on the client side (A2). Are A1 to A4 and A6 the right
rules, and is anything missing? Added by Jose Celano.

#### Positions

##### Q6 - Jose Celano (`josecelano`), 2026-10-08, PR #[To be assigned]

Restated from my review in PR #2059. A1 to A4 and A6 are required before merge, with the
acceptance criteria in [`review-pass-1.md`](review-pass-1.md). The size limit must allow every
supported key type: the 475-byte "reasonable maximum" on the I2P BitTorrent page is not a safe
universal limit. Drafted with an AI assistant using the Copilot SDK in VS Code from my review
documents.

### Q7 - Abuse controls

Even with an enforced Destination, one genuine identity can announce repeatedly, and an attacker
can run many identities. Which controls must ship with I2P support (bounded input, rate limits
keyed by trusted identity, per-swarm and global I2P peer limits, response limits, expiry), and
which belong to the shared rate-limiting design in EPIC #2411? Added by Jose Celano.

#### Positions

No positions yet.

### Q8 - REST API, logs, and privacy

Should I2P peers appear in the REST API, and in what form? Full Destinations identify peers on an
anonymity network: who may see them, and should logs and metrics redact them? Added by Jose
Celano.

#### Positions

##### Q8 - Jose Celano (`josecelano`), 2026-10-08, PR #[To be assigned]

Restated from my review in PR #2059. The existing `peer_addr` field must not carry a Destination.
I2P peers belong in a separate, explicitly typed collection, designed with an ADR in the REST API
overhaul (#144) before any implementation. Drafted with an AI assistant using the Copilot SDK in
VS Code from my review documents.

### Q9 - Testing

How is I2P support tested? The review used synthetic Destinations and `curl`. Should an end-to-end
test run a real I2P router and tunnel, and is that feasible in CI? Added by Jose Celano.

#### Positions

No positions yet.

### Q10 - Reusing the contribution

When work resumes, should it rework the PR #2059 branch, or start from the agreed design and
reuse parts of the proposal in new subissues? How is Frigyes Erdosi Szucs credited either way?
Added by Jose Celano.

#### Positions

No positions yet.

## Outcome

Pending: no decision recorded.
