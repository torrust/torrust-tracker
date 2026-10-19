---
semantic-links:
  related-artifacts:
    - "issue #2491"
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
| Scope          | The design decisions needed before the tracker supports I2P peers, using PR #2050 and its review in PR #2059 as input. UDP over I2P, I2P PEX, and I2P DHT are left out; the EPIC lists them as out of scope. WebTorrent is considered as a design input (Q12), not a deliverable |
| AI assistance  | AI assistant using the Copilot SDK in VS Code (model not recorded): research of both PRs and their branches, and the draft of this document                                                               |

## Context

### Why This Discussion

On 2026-07-31, Frigyes Erdosi Szucs (`Frigyes06`) opened PR #2050 to add I2P peer support to the
HTTP tracker. Jose Celano (`josecelano`) reviewed it, then rebased and signed it in the draft
PR #2059, keeping the contributor as author of the implementation commits and adding a review
with seven required actions.

Jose supports the feature and considers careful design of its edge cases necessary before any of
it merges. The maintainers are focused on releasing v4.0.0 (see the
[v4.0.0 milestone](https://github.com/torrust/torrust-tracker/milestone/5)), so the work is postponed
until after that release, when the roadmap is redefined.

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

##### Q1 - Cameron Garnham (`da2ce7`), 2026-10-08, PR #2500

**`addressing-as-a-layer` — Scope (Q1).** A peer is reachable at an address in some addressing layer: the IP layer (an IP address and port, IPv4 or IPv6) is one such layer, and other layers name peers in other ways. The tracker should not take on any one of them as a feature. Its first delivery is the general layer that every addressing layer needs from it, found by investigation, made general, and exported through the exposed public library API so that any project can bind an addressing layer to it, from outside the tracker or inside it. Today's IP-layer behaviour becomes the first binding of that layer. Support for an addressing layer then means that binding it is easy, and I2P is the exemplar that shows whether the layer is general enough.

AI assistance not disclosed: the author's private tooling may include machine-learning tools.

##### Q1 - Jose Celano (`josecelano`), 2026-10-19, PR #TBD

Reply to [`addressing-as-a-layer`](#q1---cameron-garnham-da2ce7-2026-10-08-pr-2500). I agree
with the goal: the tracker should not hard-code one addressing layer, and I2P is the right
exemplar, because it is the first layer whose addresses are not an IP address and port.

I disagree with building the general layer first and checking it against I2P afterwards. I would
build thin vertical slices: each slice adds a little to the I2P service, a little to the core it
needs, and a little to the tracker client, and ends with an integration test that runs through all
of them. Building every layer a little at a time finds integration problems while they are cheap
to fix, and each slice brings its own tests instead of waiting for a layer to be finished. Slicing
does not replace design: the contracts between the layers should be designed before the first
slice, because they are what keeps the slices from drifting apart.

The first slice is a proof of concept, because the solution is still uncertain: an HTTP announce
from I2P peers on its own service (see Q11), without scrape, touching the core only as far as that
announce needs. UDP over I2P has its own announce specification, separate from BEP 15, and stays
out of scope with PEX and DHT. The tracker client gains I2P support slice by slice, starting with
what the first slice's tests need.

This discussion should end with the high-level positions agreed or marked as open, topic by topic,
before the EPIC designs types, traits or package changes. A draft design is welcome here as
illustration. Drafted with an AI assistant using the Copilot SDK in VS Code (Claude); reviewed and
adopted by Jose Celano.

### Q2 - Destination trust model

A plain HTTP announce does not prove that the client owns the Destination in `ip`. An attacker can
impersonate a peer, take over its swarm record, or fill swarms with fabricated Destinations. The
[spoofing analysis](destination-spoofing-analysis.md) lists four options: reject I2P announces
until enforcement exists; accept unverified Destinations; enforce the Destination from trusted
`X-I2P-Dest*` headers added by an I2P server tunnel in front of a loopback-only listener; or a
SAMv3/I2CP adapter. Which one, and does the decision need an ADR? Added by Jose Celano.

#### Positions

##### Q2 - Jose Celano (`josecelano`), 2026-10-08, PR #2492

Restated from my review in PR #2059 and my
[PR #2050 comment](https://github.com/torrust/torrust-tracker/pull/2050#issuecomment-5327048048).
I2P announces must not be enabled while the identity comes from the `ip` query parameter. The
first secure delivery either derives the Destination from a trusted I2P server tunnel on a
dedicated, loopback-only listener, or rejects I2P announces until that exists. An unverified
compatibility mode is not acceptable for a public tracker. The decision needs an ADR. Drafted with
an AI assistant using the Copilot SDK in VS Code from my review documents.

##### Q2 - Cameron Garnham (`da2ce7`), 2026-10-08, PR #2500

**`identity-belongs-to-binding` — Address trust (Q2).** What proves that a peer owns the address it announces depends on its addressing layer: on the IP layer, the connection's source and any proxy the operator trusts are that layer's evidence, and another layer has evidence of its own. Verifying an identity therefore belongs to the binding for each layer, and the general layer must leave room for every binding to enforce identity its own way. A trust rule is a layer's evidence about its own addresses, so it belongs to that layer's binding; the general layer holds the hook that enforces a rule and never the rule.

AI assistance not disclosed: the author's private tooling may include machine-learning tools.

### Q3 - Listener configuration and `ip` precedence

Does I2P need its own listener type or per-listener mode, and how is it configured? How does a
Destination in `ip` interact with `use_ip_from_query_string` from #1987, and with
`on_reverse_proxy`? What happens to I2P identity headers on a clearnet listener? Added by Jose
Celano.

#### Positions

##### Q3 - Cameron Garnham (`da2ce7`), 2026-10-08, PR #2500

**`listener-precedence-hooks` — Listener configuration and precedence (Q3).** Two choices decide which address the tracker records for a peer: which listener the announce arrives on, and which source of the address the tracker believes, whether the connection itself, a proxy in front of it, or a value the client supplies. These are the hooks every addressing layer needs. I would make both choices general, so that each binding states where its peers arrive and which source it trusts, and today's IP-layer rules become one binding's answers rather than a fixed path through the tracker. The IP binding then answers the two questions one way and an I2P binding another, and the tracker needs neither answer in advance.

AI assistance not disclosed: the author's private tooling may include machine-learning tools.

### Q4 - Peer address model

Should `Peer::peer_addr` become a `PeerAddress` enum, as in PR #2050, or should I2P peers live in a
separate structure? The enum touches most packages and makes `Peer` non-`Copy`. It could also make
the code more generic, so that other overlay networks fit later. Added by Jose Celano.

#### Positions

##### Q4 - Jose Celano (`josecelano`), 2026-10-08, PR #2492

Restated from my
[PR #2050 comment](https://github.com/torrust/torrust-tracker/pull/2050#issuecomment-5318276034).
I see no major issue with supporting the protocol, and I think the change could make the code a
bit more generic, so that it accepts other types of peers in the future. Drafted with an AI
assistant using the Copilot SDK in VS Code from my PR comment.

##### Q4 - Cameron Garnham (`da2ce7`), 2026-10-08, PR #2500

**`address-model-at-centre` — Peer address model (Q4).** The peer's address is the centre of the general layer. The tracker should carry an address that belongs to an addressing layer rather than one that assumes an IP address and port, and expose that model through the public library API, so that each binding brings its own kind of address and its own rules for comparing and presenting it, without the tracker knowing those rules. An IP address and port is then one kind of address among others, held the same way as the rest. The opening author's entry leans the same way, toward a more generic notion of a peer; I would make that generality the deliverable, with an I2P binding beside the IP binding as the test that the model is general enough.

AI assistance not disclosed: the author's private tooling may include machine-learning tools.

##### Q4 - Jose Celano (`josecelano`), 2026-10-19, PR #TBD

This replaces my [2026-10-08 entry](#q4---jose-celano-josecelano-2026-10-08-pr-2492) and replies to
[`address-model-at-centre`](#q4---cameron-garnham-da2ce7-2026-10-08-pr-2500). I agree that a
peer's address belongs to an addressing method and that an IP address and port is one kind of
address. I no longer think one `Peer` type whose address field can hold any method's address at
run time, as PR #2050's `PeerAddress` enum does, is the right shape. Peers are never offered across
addressing methods (see my Q5 entry), so the enum would make `Peer` larger, cost it `Copy`, and put
a tag check on every IP operation for no benefit, and it would touch the HTTP and UDP trackers to
add something they never use.

The shared core keeps one group of peers per addressing method, each with its own peer type. The
proof of concept of my Q1 and Q11 entries adds a peer type and a group for I2P Destinations and
leaves the IP `Peer` unchanged. Once two concrete groups exist, the extraction step decides what
they share: probably one structure that is generic over the peer's address, so the IP path pays
nothing for the others. A related refactor is to split today's `Peer` into its address and the
announce data common to every method (peer ID, the bytes uploaded, downloaded and left, the event,
the time of the last announce); the swarm already stores the address twice, as the map key and
inside the value. That split is a breaking change to `torrust-tracker-primitives` and should be
checked against WebTorrent first (see Q12), where a peer is never offered by its network address. How an address
is presented on the wire belongs to each service's protocol package, so the core needs only an
address it can store and compare. Drafted with an AI assistant using the Copilot SDK in VS Code
(Claude); reviewed and adopted by Jose Celano.

### Q5 - Swarm isolation and statistics

PR #2050 keeps one swarm per info hash and filters peers by network when it answers. Is that
enough, or should each network have its own swarm? Announce responses must count only reachable
peers (A5), but what should scrape responses, the REST API, and metrics count? Should
`max_peers_per_announce` and `numwant` apply per network? Added by Jose Celano.

#### Positions

##### Q5 - Cameron Garnham (`da2ce7`), 2026-10-08, PR #2500

**`reachability-decided-once` — Swarm isolation and statistics (Q5).** Which peers are offered to which, and what the counts report, should be decided once in the general layer and hold for every addressing layer. The layer owns the rule that a peer is offered only peers it can reach, and the counts that announce, scrape and the application API report under that rule. Each binding declares which of its addresses can reach which; on the IP layer, for instance, that is a question between IPv4 and IPv6 peers. Limits on how many peers an answer carries belong to the layer as well, so that no binding invents them again, and a binding adds statistics of its own only where they mean something to its layer alone.

AI assistance not disclosed: the author's private tooling may include machine-learning tools.

##### Q5 - Jose Celano (`josecelano`), 2026-10-19, PR #TBD

Reply to [`reachability-decided-once`](#q5---cameron-garnham-da2ce7-2026-10-08-pr-2500). I would
separate two decisions that are easy to mix up: how the core stores peers, and which peers a
service returns.

**Storage.** There is one swarm per info hash, in the shared core, for every service. The
[shared-services ADR](../../../adrs/20260727180000_shared_services_across_tracker_instances.md)
already rejects a peer repository per tracker instance, because the reason to run several
listeners is to serve the same swarm: a peer that announces to one listener is returned by every
other listener that can use its address, whatever its protocol. Inside that swarm, peers are kept
in one group per addressing method: IPv4, IPv6, I2P Destinations, and later WebTorrent (see Q12).
Separate groups reduce contention, so an I2P or WebTorrent announce does not compete with IP
traffic for the same peer list, and the IPv4/IPv6 split is already planned as a performance
improvement
([discussion #774](https://github.com/torrust/torrust-tracker/discussions/774#discussioncomment-8999151)).
Partitioning is not isolation: no group belongs to one listener.

**Delivery.** Each service decides which groups it reads, within what its protocol allows. The HTTP
tracker reads both IP groups and returns them in `peers` and `peers6`, as it does today, so the
split need not change its behaviour; reading two groups costs two lookups. The UDP tracker reads
the requester's address family only, because BEP 15 picks the response format from the family of
the request packet (#1366). The I2P service reads Destinations only, because the I2P specification
forbids returning clearnet peers, and a WebTorrent service reads only WebTorrent peers. This is
where I differ from the entry: the rule of who may receive whom belongs to each service's protocol,
not to one rule in the general layer, while the core owns the groups, the selection of peers within
a group, and the limits such as `max_peers_per_announce` and `numwant`.

This rules out PR #2050's shape, one peer list per info hash with I2P and IP peers filtered on
every read. One question stays open: what the counts report. Today `complete`, `incomplete`, scrape
and the `downloads` counter cover both IP families of an info hash. For an I2P or WebTorrent client
they should probably count only its own group, while the REST API and metrics can report per group
and in total; the persisted `downloads` counter needs a decision of its own. Drafted with an AI
assistant using the Copilot SDK in VS Code (Claude); reviewed and adopted by Jose Celano.

### Q6 - Wire format and input handling

These rules decide what the tracker accepts and returns: percent-decoding (A1); the rule that
identifies a suffixless Destination (A6); the maximum Destination size and the supported
certificate types (A4); structured errors that never echo the Destination (A3); the placeholder
port (F2); and compact response parsing on the client side (A2). Are A1 to A4 and A6 the right
rules, and is anything missing? Added by Jose Celano.

#### Positions

##### Q6 - Jose Celano (`josecelano`), 2026-10-08, PR #2492

Restated from my review in PR #2059. A1 to A4 and A6 are required before merge, with the
acceptance criteria in [`review-pass-1.md`](review-pass-1.md). The size limit must allow every
supported key type: the 475-byte "reasonable maximum" on the I2P BitTorrent page is not a safe
universal limit. Drafted with an AI assistant using the Copilot SDK in VS Code from my review
documents.

##### Q6 - Cameron Garnham (`da2ce7`), 2026-10-08, PR #2500

**`encoding-belongs-to-binding` — Wire format and input handling (Q6).** Each addressing layer has its own written form of an address. The rules for that form, how it is decoded, bounded, compared and reported in errors without leaking it, belong to the binding for that layer, as the rules for writing an IP address and port belong to the IP binding. The rules for how the tracker reads any request, before it knows which layer an address belongs to, belong to the general layer and hold for every binding alike.

AI assistance not disclosed: the author's private tooling may include machine-learning tools.

### Q7 - Abuse controls

Even with an enforced Destination, one genuine identity can announce repeatedly, and an attacker
can run many identities. Which controls must ship with I2P support (bounded input, rate limits
keyed by trusted identity, per-swarm and global I2P peer limits, response limits, expiry), and
which belong to the shared rate-limiting design in EPIC #2411? Added by Jose Celano.

#### Positions

##### Q7 - Cameron Garnham (`da2ce7`), 2026-10-08, PR #2500

**`controls-split-by-identity` — Abuse controls (Q7).** Controls that bound what any announce can cost the tracker, such as the size of what it reads and returns, peer limits per swarm and overall, and expiry, do not depend on how peers are addressed. They belong to the general layer, apply to every binding alike, and meet the shared rate-limiting design of EPIC #2411 there. Controls keyed by an identity depend on what one identity is, which only the binding can say: an IP address on one layer, something else on another. The layer's part is to give each binding a place to apply its own keyed controls and to account for them beside the general ones.

AI assistance not disclosed: the author's private tooling may include machine-learning tools.

### Q8 - REST API, logs, and privacy

Should I2P peers appear in the REST API, and in what form? Full Destinations identify peers on an
anonymity network: who may see them, and should logs and metrics redact them? Added by Jose
Celano.

#### Positions

##### Q8 - Jose Celano (`josecelano`), 2026-10-08, PR #2492

Restated from my review in PR #2059. The existing `peer_addr` field must not carry a Destination.
I2P peers belong in a separate, explicitly typed collection, designed with an ADR in the REST API
overhaul (#144) before any implementation. Drafted with an AI assistant using the Copilot SDK in
VS Code from my review documents.

##### Q8 - Cameron Garnham (`da2ce7`), 2026-10-08, PR #2500

**`library-api-exports-the-layer` — The exposed public library API.** The exported library surface that a project links against to bind an addressing layer, from outside the tracker or inside it. Its shape for peers is the general address model, with an IP address and port as one kind of address, and every binding presents its addresses through it without a field of its own. It is consumed by linking against a given version: it is the library's public items, and no stable binary shape is promised.

**`application-api-presents-the-layer` — The application API.** What the running tracker exposes on top of that export, in every shape it takes: the REST API (torrents and their peers, the whitelist, authentication keys, statistics in JSON and Prometheus form, the health check), the health-check API server and the logs. It is a consumer of the export, never a second definition of the address model.

**`api-exports-the-layer` — Application API and privacy (Q8).** What an address may reveal, and whether the application API's shapes redact it, depends on its layer, so each binding decides it and the layer carries that choice through every shape of the application API. The opening author's entry places the representation in the REST API overhaul as a separately typed collection; I would have that overhaul present the general address model, of which IP-layer peers and I2P peers are two cases.

AI assistance not disclosed: the author's private tooling may include machine-learning tools.

##### Q8 - Jose Celano (`josecelano`), 2026-10-19, PR #TBD

Reply to [`api-exports-the-layer`](#q8---cameron-garnham-da2ce7-2026-10-08-pr-2500). I agree
with naming the library API and the application API separately, and with the rule that the
application API presents the address model and never defines a second one. I keep my earlier
position on the shape: with one group of peers per addressing method (my Q5 entry), the REST API
presents each group's peers as its own typed collection, which is what I asked for in PR #2059, and a
Destination never travels in a field typed for a socket address. Whether logs and metrics redact a
Destination is a question for the I2P binding, as the entry says; the general rule I would add is
that no surface prints a full Destination by default. Drafted with an AI assistant using the
Copilot SDK in VS Code (Claude); reviewed and adopted by Jose Celano.

### Q9 - Testing

How is I2P support tested? The review used synthetic Destinations and `curl`. Should an end-to-end
test run a real I2P router and tunnel, and is that feasible in CI? Added by Jose Celano.

#### Positions

##### Q9 - Cameron Garnham (`da2ce7`), 2026-10-08, PR #2500

**`layer-tested-independently` — Testing (Q9).** The general layer is tested on its own terms, through the library API it exports, with more than one binding: the IP binding shows that today's behaviour holds on the new layer, and a second binding shows that the layer is general. How a binding tests its own addressing layer, including whether its tests need that layer's real network, is the binding's question; for an I2P binding beside the IP binding, that includes whether a real router can run in CI.

AI assistance not disclosed: the author's private tooling may include machine-learning tools.

### Q10 - Reusing the contribution

When work resumes, should it rework the PR #2059 branch, or start from the agreed design and
reuse parts of the proposal in new subissues? How is Frigyes Erdosi Szucs credited either way?
Added by Jose Celano.

#### Positions

##### Q10 - Cameron Garnham (`da2ce7`), 2026-10-08, PR #2500

**`proposal-as-next-binding` — Reusing the contribution (Q10).** I would start from the agreed design rather than rework the proposal's branch, because the deliverable has changed shape: the general layer comes first, with today's IP-layer behaviour as its first binding, and the proposal's knowledge of its addressing layer becomes the next binding, inside the tracker or outside it, whichever the layer makes easier. Its parsing, encoding and isolation work is the most direct evidence of what the layer must offer an addressing layer that is not IP, so the layer's design should be checked against it before it is settled. That check is what using I2P as the exemplar means.

AI assistance not disclosed: the author's private tooling may include machine-learning tools.

### Q11 - HTTP tracker mode or separate I2P tracker service

Should I2P support be a mode of the existing HTTP tracker, sharing its listener, handlers and code
paths, or a separate tracker service that happens to speak the same HTTP announce protocol? Added
by Jose Celano.

#### Positions

##### Q11 - Jose Celano (`josecelano`), 2026-10-19, PR #TBD

Both choices have a cost. As a mode of the HTTP tracker, every path that handles a peer's address
gains a condition for the two modes. As a separate service, much of the HTTP tracker's code is
duplicated, because the two speak the same announce protocol. I lean towards a separate service,
because the two look alike but are not:

- **Different specifications.** BEP 3 and BEP 23 belong to BitTorrent.org; the I2P BitTorrent
  specification belongs to the I2P project, which already changes the announce `ip` parameter, the
  compact peer format and the source of the client's identity. Either organisation can change its
  specification without regard to the other.
- **Different reasons to change.** The HTTP tracker is one of the services users ask for most. It
  should stay stable, mature and well tested. I2P support is experimental. Keeping them apart
  means a change made for one cannot break the other.
- **Little of the delivery layer is shared.** `axum-http-server` is already a thin layer, and much
  of it is specific to IP addresses. Resolving the client's address from the connection or from
  `X-Forwarded-For` (`client_ip_sources.rs`) does not apply: an I2P peer's identity comes from the
  headers that the I2P server tunnel adds.

The strategy I would follow:

1. Add a new service to the tracker's orchestration, with its own configuration section and its
   own listener. A dedicated listener is also what Q2 needs for the loopback-only tunnel.
2. Give the I2P service its own full stack except the tracker core: its own protocol, core and
   server packages, built for the first version by copying the HTTP tracker's packages and
   removing what does not apply to I2P or is not needed yet, such as client IP resolution. The
   tracker core is shared by every service and keeps one group of peers per addressing method,
   Destinations apart from IP addresses; each service chooses which groups it returns (see my Q5
   entry). Splitting IPv4 from IPv6 peers the same way is already planned as a performance
   improvement
   ([discussion #774](https://github.com/torrust/torrust-tracker/discussions/774#discussioncomment-8999151)).
   Authentication keys and the whitelist already live in the shared core, so the I2P service can
   use them later; the first slice leaves them out, together with scrape.
3. Re-evaluate the remaining code and move what the HTTP tracker and the I2P tracker share
   completely into shared packages.

Some duplication is deliberate. The announce query looks the same for both services but carries
conflicting domain rules: the HTTP tracker's request type classifies the `ip` parameter as an IP
literal, a DNS name or an invalid value, and has no place for a Destination, while the I2P
service's request type must accept a Destination and nothing else. With one shared type, a
deployment that runs both services could let one of them accept an address that belongs to the
other. Separate types reject it when the query is decoded, which is the earliest point it can be
caught.

This is the proof of concept of my Q1 entry. What the I2P service needs from the shared core, today
built around a peer with an IP address and port, is the question of Q4 and Q5.

Another tracker faces the same choice. In aquatic's open
[I2P support issue](https://github.com/greatest-ape/aquatic/issues/248), a contributor weighs one
general host address type for every network, which they call a fundamental refactor, against an
additional stack for I2P alongside the IP stacks. The maintainer has not had time to take it on, so
the issue does not record a decision, but it shows the same trade-off between Cameron's general
layer and a separate service. Drafted with an AI assistant using the Copilot SDK in VS Code
(Claude); reviewed and adopted by Jose Celano.

### Q12 - WebTorrent as a second design input

WebTorrent (BitTorrent over WebRTC, for browsers) is another way to address peers, with its own
tracker protocol over WebSocket, and the project once considered supporting it (#15). I2P and
WebTorrent would be delivered one at a time, but both affect the same parts of the architecture.
Should the design of the shared core and the services be checked against WebTorrent as well as
I2P, even if WebTorrent is not delivered by EPIC #2491? Added by Jose Celano.

#### Positions

##### Q12 - Jose Celano (`josecelano`), 2026-10-19, PR #TBD

Yes. Designing for both is cheaper than reworking the core for the second one. What follows comes
from two implementations: [`webtorrent/bittorrent-tracker`](https://github.com/webtorrent/bittorrent-tracker),
the reference tracker from the organisation that defined the protocol, and aquatic's `ws_protocol`
and `ws` crates.

A WebTorrent tracker does two things. It keeps a peer list, as every tracker does, and it relays
messages between peers. An announce carries no address the peer can be reached at: it carries
WebRTC offers, or an answer to an offer that another peer sent. The tracker forwards each offer to
a randomly chosen peer of the swarm over that peer's open WebSocket connection and relays the answer
back. The announce response carries only the counts and the interval, never a peer list, and the
reference tracker gives WebSocket clients an interval five times shorter than HTTP clients.

Both implementations already keep the two responsibilities apart. In the reference tracker the
swarm only stores peers and picks them: for a WebSocket announce it picks as many peers as the
announce carries offers, and the WebSocket server sends the offers and relays the answers. In
aquatic a swarm worker owns the peer list and returns messages addressed to connections, and socket
workers own the connections and deliver them. I would keep that separation: the peer list belongs
to the shared core like any other group (see my Q5 entry), keyed by peer ID, as both
implementations key it, and the relay belongs to the WebTorrent service. Choosing N peers of a group
excluding the requester is the same core operation for every service; for WebTorrent N is the number
of offers. A WebTorrent peer is then reached through the connection the tracker holds for it, not
through an address the other peer connects to.

Four points couple the two more than for the other methods, and the design has to place each one:

1. **Lifecycle.** A peer leaves when its connection closes, not only when it expires or announces
   `stopped`. The reference tracker remembers which swarms each connection joined and, when the
   connection closes, announces `stopped` for it in each of them, so the core needs no operation of
   its own for this.
2. **Several listeners.** The shared-services ADR lets a peer on one WebTorrent listener be offered
   to a peer on another, so the handle to a peer's connection must name the listener that holds it,
   and the service must route messages between listeners. Aquatic does the same between its socket
   workers inside one process.
3. **Pending offers.** Aquatic relays an answer only to the peer whose offer it forwarded, so it
   remembers the offers waiting for an answer in the peer entry. The reference tracker relays an
   answer to any peer of the swarm named in it. The check is hardening that can live in the
   WebTorrent service.
4. **Identity.** The connection is the evidence of a peer's identity. Aquatic ignores an announce
   that reuses a peer ID from a different connection; the reference tracker does not check it. That
   is a trust rule of the kind Q2 places in each addressing method.

A global swarm grouped by addressing method still fits: WebTorrent peers are one more group, and
the WebTorrent service reads only that group. Mixing groups has no use, because a browser peer can
connect only over WebRTC and a WebTorrent response carries no peer list; a client that speaks both
announces to both kinds of tracker and appears in both groups. The reference tracker shows both
sides of the counting question of Q5: it keeps HTTP, UDP and WebTorrent peers in one swarm per info
hash and filters them by type when it picks peers, which is the shape my Q5 entry rules out for
performance, and its counts cover every type, so a WebTorrent client is told about seeders it
cannot reach. Aquatic reports each group's own counts. The reference tracker also keeps the IP
address of each WebSocket connection, honouring `X-Forwarded-For` when it trusts a proxy, for its
statistics; the core can use it for controls keyed by IP address (Q7) even though it is never
offered to other peers. Aquatic splits WebTorrent peers by the IP version of their connection, and
the reference tracker does not. WebTorrent has no specification for IPv6, and a member of the
WebTorrent community told me on the project's Discord that peers connected over IPv4 should receive
peers connected over IPv6, which would make aquatic's split a bug. I have asked aquatic for its
reason ([greatest-ape/aquatic#262](https://github.com/greatest-ape/aquatic/issues/262)) and would
not copy the split without one. Drafted with an AI assistant using the Copilot SDK in VS Code
(Claude); reviewed and adopted by Jose Celano.

## Outcome

Pending: no decision recorded.
