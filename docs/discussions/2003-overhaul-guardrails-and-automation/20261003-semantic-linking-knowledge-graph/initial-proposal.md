# Semantic Linking and Knowledge Graphs for the Torrust Tracker Repository

> **Source material, not policy.** Written by an AI agent in a conversation with Jose Celano on
> 2026-10-03. The text is kept as written; only heading levels were changed to pass the Markdown
> linter. The review and draft conclusions are in [`README.md`](README.md).

## 1. The problem

The Torrust Tracker repository contains many different kinds of artifacts that are meaningful from a semantic point of view:

- source code
- Rust modules, structs, functions, methods and tests
- issue specifications
- Markdown documents
- Markdown sections
- ADRs
- behavioral / acceptance tests
- requirements
- implementation notes
- decisions
- commits and pull requests
- historical material

The long-term goal is to be able to build a **knowledge graph of the repository** in which these things can be connected semantically.

For example:

```text
Issue specification
        │
        ├── relates to ──> Tracker::remove_peer
        │
        ├── motivated by ──> ADR-0042
        │
        ├── implemented by ──> PR #1234
        │
        └── tested by ──> acceptance test
```

The difficulty is that the repository is primarily a collection of artifacts intended for humans, not a database.

The current linking approach is rudimentary: links can be embedded in many places, but there is no unified representation of links or resources.

A second problem appears when trying to link to something fine-grained, such as a Markdown section. A conventional approach requires giving every section an explicit unique identifier:

```markdown
## Connection handling {#connection-handling-123}
```

That creates friction for authors. It also encourages the repository to become progressively more rigid and structured simply to make it machine-readable.

The question is therefore:

> **How can we create a reliable semantic graph of the repository without forcing all repository artifacts, especially exploratory specifications, into rigid machine-oriented formats?**

---

## 2. Do not make Markdown the data model

One tempting solution is to turn specifications into structured Rust types:

```text
Markdown
   ↓
structured specification
   ↓
Rust types
   ↓
JSON
   ↓
Markdown generated from templates
```

This has obvious advantages for machines:

- explicit fields
- validation
- stable identifiers
- easy serialization
- easy database ingestion
- predictable structure

But it also has a significant cost.

A specification is often an evolving human artifact. During implementation, people discover things, change their understanding, reject approaches, add context, and explore alternatives.

Forcing all of that into a rigid schema means that the specification language itself starts becoming a program.

Eventually:

```text
natural-language specification
        ↓
structured data
        ↓
formal specification language
        ↓
executable specification
```

The specification can begin to resemble a second implementation of the production program.

That is not necessarily wrong in domains where formal specification is the explicit objective. But it is undesirable if the purpose of an issue specification is to provide a flexible working space for humans during implementation.

The better principle is:

> **Do not make every artifact structured. Make the relationships between artifacts structured.**

---

## 3. Disposable specifications versus definitive specifications

An important distinction is between an **initial issue specification** and a **durable specification of system behavior**.

An issue specification is often disposable.

It represents what the developers currently think should be done:

```text
What problem are we solving?
What do we currently understand?
What are the possible approaches?
What constraints have we discovered?
What should we investigate?
```

During implementation, the understanding changes.

The final implementation may therefore differ substantially from the initial specification.

This suggests a lifecycle like:

```text
                    implementation
                         │
                         ▼
                ┌─────────────────┐
                │  Issue spec     │
                │                 │
                │ exploratory     │
                │ provisional     │
                │ contextual      │
                └────────┬────────┘
                         │
                         ▼
                    implementation
                         │
              ┌──────────┴──────────┐
              ▼                     ▼
          source code        behavioral tests
                                    │
                                    ▼
                           durable specification
```

The issue specification answers:

> **What did we currently think we were trying to do?**

The behavioral tests answer:

> **What behavior does the system actually promise?**

Those are different questions.

The first is a working artifact.

The second can become a durable specification.

---

## 4. High-level behavioral tests as the durable specification

For behavior that must remain stable over time, executable high-level tests are particularly valuable.

A specification can initially say:

```markdown
The tracker should reject invalid peer addresses before
creating the corresponding connection.
```

During implementation, this can eventually become an acceptance test expressing the actual required behavior.

The long-term source of truth is then not necessarily the original prose but the behavior that the system must continue to satisfy.

This avoids the requirement that the original issue specification must itself become a perfectly formal representation.

The issue can even contain the intended acceptance test in prose or as an initial programmable test:

```markdown
The implementation should eventually satisfy an acceptance test
which verifies that invalid peer addresses are rejected before
a connection is created.
```

or:

```rust
#[test]
fn it_should_reject_invalid_peer_address_before_creating_connection() {
    // ...
}
```

The specification can therefore evolve naturally toward executable behavior without requiring the entire initial document to become a program.

---

## 5. Frontmatter should be rigid; the body should remain flexible

A good compromise is:

```markdown
---
type: issue-spec
issue: 1525
status: implementing
created: 2026-10-03
---

# Persistence overhaul

The current persistence architecture has several problems...

## Investigation

...

## Possible approaches

...

## Proposed direction

...
```

The **frontmatter** is structured.

The **body** remains ordinary Markdown.

This gives us a useful separation:

```text
┌──────────────────────────────┐
│ Frontmatter                  │
│                              │
│ rigid metadata               │
│ identity                     │
│ status                       │
│ dates                        │
│ relationships if appropriate │
└──────────────┬───────────────┘
               │
               ▼
┌──────────────────────────────┐
│ Markdown body                │
│                              │
│ flexible                     │
│ exploratory                  │
│ contextual                   │
│ human-oriented               │
└──────────────────────────────┘
```

The machine-readable part is deliberately limited.

There is no need to turn every paragraph into a structured object.

---

## 6. The knowledge graph should not require semantic parsing of prose

This is the central idea.

Suppose an issue contains:

```markdown
## Proposed solution

We should validate the address before creating the connection.

The validation should be shared with
[[src/network.rs::validate_address]].

## Notes

I initially considered moving this into...
```

A knowledge graph system does not need to understand the meaning of the surrounding English.

It only needs to recognize that a link exists:

```text
issue-123#proposed-solution
        │
        └── references ──> src/network.rs::validate_address
```

The Markdown parser therefore does **not** need to become a semantic parser for issue specifications.

It only needs to extract explicit references.

This is much simpler.

The parser's job is:

```text
Markdown
   ↓
find explicit links
   ↓
resolve references
   ↓
produce graph edges
```

not:

```text
Markdown
   ↓
understand all prose
   ↓
classify every sentence
   ↓
extract requirements
   ↓
build a formal specification
```

---

## 7. Addressability and identity are different things

The problem with Markdown sections comes from treating addressability and identity as the same concept.

Consider:

```markdown
## Connection handling
```

There is already an obvious address for this section:

```text
document + heading
```

It does not necessarily need a manually assigned identifier.

The graph system could internally represent it as:

```text
document:
    docs/issues/1234.md

selector:
    heading("Connection handling")
```

or conceptually:

```text
docs/issues/1234.md#heading:connection-handling
```

The important point is that this does **not** require the author to write the identifier into the Markdown.

The graph system derives it.

Therefore:

> **An address can be generated by the graph system without becoming an author-maintained identifier.**

---

## 8. Different resources have different kinds of identity

The repository contains fundamentally different entities:

```text
Issue
ADR
Markdown document
Markdown section
Rust module
Rust function
Rust method
Rust struct
Rust enum
Test
Commit
Pull request
Behavior
Requirement
```

It would be a mistake to force all of them into a single manually assigned identifier scheme.

Instead, define a generic concept such as:

```text
ResourceRef
    kind
    locator
```

Examples:

```text
issue:1525

adr:42

file:src/tracker.rs

rust:function:src/tracker.rs::Tracker::start

test:tests/acceptance/foo.rs::it_should_accept...

markdown:docs/issues/1234.md#heading:connection-handling
```

These resources can have different identity mechanisms.

An issue might have an intrinsic identity:

```text
repository + issue number
```

A file has:

```text
repository + path
```

A Rust method can be resolved from the Rust syntax tree:

```text
crate + module + type + method
```

A Markdown section can be addressed by:

```text
document + structural selector
```

The graph infrastructure can unify these concepts without forcing authors to manually assign IDs to everything.

---

## 9. The linking syntax should be independent of the relationship semantics

It is useful to distinguish between:

1. **reference syntax**
2. **resource resolution**
3. **semantic relationship**

For example:

```markdown
This implements [[requirement:REQ-123]].
```

The `[[...]]` syntax establishes a reference.

The resolver turns the reference into a resource.

Only then should the system determine or record the relationship.

Conceptually:

```text
[[requirement:REQ-123]]
        │
        ▼
resource resolution
        │
        ▼
requirement:REQ-123
```

Then the graph can contain:

```text
source ──> target
```

with an optional relationship:

```text
source ──implements──> target
```

This separation is valuable because the repository does not need to decide its entire semantic ontology in advance.

---

## 10. Start with simple links

The first version could deliberately be very simple.

For example:

```markdown
[[Tracker::remove_peer]]
[[issue:1525]]
[[adr:42]]
[[file:src/tracker.rs]]
[[./other.md#Investigation]]
[[#Connection handling]]
```

The syntax only establishes:

> There is a reference to this resource.

The system can initially produce:

```text
source → target
```

with information about where the link occurred.

Later, relationships can become richer:

```text
source ──implements──> target

source ──tested-by──> target

source ──motivated-by──> target

source ──supports──> target

source ──contradicts──> target

source ──supersedes──> target

source ──derived-from──> target
```

The ontology can evolve from real usage rather than being designed completely in advance.

---

## 11. The parser should extract links, not understand specifications

This distinction is particularly important for issue specs.

Consider:

```markdown
## Problem

The tracker currently creates a connection before validating
the peer address.

This creates a race with...

## Proposed solution

We should validate the address before creating the connection.

The validation should be shared with
[[Tracker::validate_address]].

## Notes

I initially considered moving this into...
```

The parser does not need to determine:

- what the problem is
- what the proposed solution is
- what is a requirement
- what is an observation
- what is historical
- what is speculative

It only needs to determine:

```text
This document contains a reference to Tracker::validate_address.
```

If somebody explicitly wants the heading itself to participate in the graph, then:

```text
issue-123#Proposed solution
```

can become a resource.

Otherwise it does not need to become a graph node at all.

---

## 12. Not every discovered thing needs to be a graph node

This is another useful constraint.

Suppose an issue says:

```markdown
The problem appears to originate in the connection cleanup
performed by [[Tracker::remove_peer]].
```

The important graph relationship is:

```text
issue-123
    ──mentions──> Tracker::remove_peer
```

If another section says:

```markdown
## Investigation

The cleanup is actually triggered by
[[Tracker::handle_disconnect]].
```

then:

```text
issue-123#Investigation
    ──mentions──> Tracker::handle_disconnect
```

The heading only needs to become a graph node because something needs to refer to it.

This suggests a useful principle:

> **Things become graph entities because they participate in relationships, not merely because a parser discovered them.**

This keeps the graph smaller and more meaningful.

---

## 13. Implicit locations are preferable to manually assigned IDs

A useful syntax is to allow references relative to the current document.

For example:

```markdown
[[#Connection handling]]
```

means:

> The "Connection handling" section of this document.

And:

```markdown
[[./other.md#Investigation]]
```

means:

> The "Investigation" section of `other.md`.

The author never needs to create:

```markdown
## Connection handling {#connection-handling-8f4a}
```

The graph resolver can derive:

```text
repository:
    torrust-tracker

document:
    docs/issues/1234.md

selector:
    heading("Connection handling")
```

This is an **address**, not a manually maintained ID.

---

## 14. Derived identities can legitimately change

There is a caveat.

A heading-based identity is not necessarily stable.

If:

```markdown
## Connection handling
```

becomes:

```markdown
## Peer connection handling
```

then the derived address changes.

For disposable issue specifications, this is not necessarily a problem.

In fact, it may be useful.

The graph can discover:

```text
old reference → unresolved
```

and report that something has become stale.

This gives the graph another useful function:

> It can identify broken or obsolete semantic relationships.

For resources whose identity genuinely must survive renaming, an explicit identity can be introduced.

This gives a spectrum:

```text
                     stability
                         ↑

explicit ID              │   requirement:REQ-42

intrinsic identity       │   Tracker::remove_peer

derived address          │   issue-1525#heading:foo

physical location        │   src/foo.rs

                         └──────────────────────→
```

Not every entity needs the same degree of identity.

---

## 15. A generic resource resolver

The core abstraction should therefore be a **resource resolver**.

Conceptually:

```rust
ResourceRef
    ├── Issue
    ├── File
    ├── MarkdownDocument
    ├── MarkdownSection
    ├── RustSymbol
    ├── Test
    ├── ADR
    └── ExternalResource
```

This does not mean the repository's documents need to become Rust structs.

The graph infrastructure has the model.

The documents do not.

For example:

```text
"Tracker::remove_peer"
        │
        ▼
Rust syntax-tree index
        │
        ▼
Rust symbol node
```

while:

```text
"#Connection handling"
        │
        ▼
current Markdown document
        │
        ▼
Markdown section node
```

The human representation stays natural while the infrastructure provides machine-level resolution.

---

## 16. The repository architecture

A possible architecture is:

```text
                         Repository
                             │
          ┌──────────────────┼──────────────────┐
          │                  │                  │
          ▼                  ▼                  ▼
   Markdown parser       Rust analyzer      Git / Issue API
          │                  │                  │
          ▼                  ▼                  ▼
 Markdown resources     Rust resources     Issue resources
          │                  │                  │
          └──────────────────┼──────────────────┘
                             ▼
                    ┌──────────────────┐
                    │ Resource index   │
                    └────────┬─────────┘
                             │
                       resolve [[...]]
                             │
                             ▼
                    ┌──────────────────┐
                    │ Semantic edges   │
                    │                  │
                    │ source           │
                    │ target           │
                    │ location         │
                    │ relation?        │
                    │ evidence          │
                    └────────┬─────────┘
                             ▼
                    Knowledge graph DB
```

The Markdown parser only needs enough Markdown understanding to:

- recognize links
- locate the surrounding document
- optionally identify headings
- provide source locations

It does not need to understand the semantic content of the prose.

---

## 17. A link is evidence

There is another useful concept here: a link should retain its **evidence**.

Instead of storing only:

```text
A → B
```

store something closer to:

```text
Edge
    source: A
    target: B
    relation: implements
    source_artifact: docs/issues/1234.md
    source_location: line 87
    syntax: [[Tracker::remove_peer]]
```

This matters because the graph is not necessarily authoritative.

It is an interpretation of evidence contained in the repository.

You can therefore distinguish:

```text
graph assertion
```

from:

```text
source evidence
```

If an edge becomes questionable, the system can take you directly back to the artifact that created it.

---

## 18. Explicit links versus inferred links

Eventually, the system may want to infer relationships.

For example:

```text
Issue 123
    mentions Tracker::remove_peer

PR 456
    modifies Tracker::remove_peer

PR 456
    closes Issue 123
```

The system might infer:

```text
Issue 123
    ──possibly-implemented-by──> PR 456
```

But this should initially remain distinct from an explicit statement:

```text
PR 456
    ──implements──> Issue 123
```

This suggests another useful distinction:

```text
explicit edge
inferred edge
historical edge
```

The graph can then contain richer information without pretending that every inference is an explicit fact.

---

## 19. The knowledge graph should be allowed to be incomplete

There is a temptation to make the graph comprehensive:

> Every requirement must be linked.
>
> Every implementation must have a requirement.
>
> Every test must have a specification.
>
> Every paragraph must have a semantic classification.

That would recreate the original problem.

The graph should instead be **selectively explicit**.

A useful principle is:

> **If a relationship is important enough to query later, make it explicit. If it isn't, leave the artifact alone.**

This means a developer can write normal prose and only add links where the connection has semantic value.

---

## 20. This fits the Naur perspective

The approach also fits well with the idea behind Peter Naur's *Programming as Theory Building*.

The theory of a system is not equivalent to:

```text
source code
```

nor:

```text
formal specification
```

nor:

```text
documentation
```

It is distributed across:

```text
source code
tests
decisions
explanations
historical context
design discussions
implementation experience
terminology
relationships between concepts
```

The knowledge graph should therefore not attempt to become the complete theory itself.

Instead, it can become a **navigation structure through the artifacts that collectively contain the theory**.

Conceptually:

```text
                   Project theory
                         │
          ┌──────────────┼───────────────┐
          ▼              ▼               ▼
       source          tests          documents
          │              │               │
          └──────────────┼───────────────┘
                         │
                         ▼
                  semantic links
                         │
                         ▼
                  knowledge graph
```

The graph is therefore a map of the distributed theory, not a replacement for it.

---

## 21. This also avoids creating a "mirror program"

There is a dangerous trajectory:

```text
natural-language specification
        ↓
structured JSON
        ↓
Rust structs
        ↓
validation
        ↓
code generation
        ↓
production program
```

Eventually:

```text
specification ≈ second implementation
```

At that point, the specification language may have almost the same complexity as the production program.

The alternative is:

```text
                  human theory
                       │
          ┌────────────┼────────────┐
          ▼            ▼            ▼
     issue spec       code       behavioral tests
     disposable     evolving        durable
          │            │               │
          └────────────┼───────────────┘
                       │
                       ▼
                semantic graph
```

The graph does not make the issue specification formal.

It records the connections between artifacts.

---

## 22. A possible unified link syntax

A minimal system could start with something like:

```markdown
[[issue:1525]]

[[adr:42]]

[[Tracker::remove_peer]]

[[file:src/tracker.rs]]

[[./other.md]]

[[./other.md#Investigation]]

[[#Connection handling]]
```

The exact syntax is not the important part.

The important architectural properties are:

1. It works in Markdown.
2. It can also work in comments or other text-oriented artifacts.
3. It does not require every resource to have a manually assigned ID.
4. The resolver can determine the canonical resource.
5. The graph stores the canonical resource separately from the textual reference.
6. The relationship semantics can evolve independently of the syntax.

---

## 23. A useful internal representation

The graph could eventually represent something like:

```rust
struct Resource {
    id: ResourceId,
    kind: ResourceKind,
    locator: Locator,
}

struct Edge {
    source: ResourceId,
    target: ResourceId,
    relation: Option<Relation>,
    evidence: Evidence,
}

struct Evidence {
    artifact: ResourceId,
    location: SourceLocation,
    explicit: bool,
}
```

The exact Rust representation is not important at this stage.

The conceptual separation is:

```text
Resource
    =
    something that can be referred to

Reference
    =
    textual way of referring to it

Edge
    =
    relationship between resources

Evidence
    =
    where the relationship came from
```

This separation is likely to remain useful even if the eventual graph database is completely different from the initial implementation.

---

## 24. Do not design the complete ontology first

One of the most important constraints should be:

> **Do not design the complete knowledge graph ontology before collecting real links from the repository.**

Start with actual Torrust material.

For example:

```text
issue-1525
    → src/persistence/mod.rs

issue-1525
    → ADR-...

issue-1525
    → acceptance-test-...

acceptance-test-...
    → Tracker::...

Tracker::...
    → module...

module...
    → ADR...
```

After a few thousand real links exist, patterns will emerge.

You may discover that the repository naturally needs relationships such as:

```text
implements
tests
motivated-by
depends-on
supersedes
contradicts
explains
mentions
derived-from
```

Or you may discover that some of those distinctions are artificial.

That discovery should come from the repository rather than from an abstract ontology designed in advance.

---

## 25. The first implementation can therefore be intentionally stupid

A very reasonable first implementation is:

```text
1. Scan repository.
2. Index known resources.
3. Parse Markdown enough to find [[...]] references.
4. Parse Rust enough to index symbols.
5. Resolve references.
6. Store source → target edges.
7. Keep source location as evidence.
```

Initially:

```text
source → target
```

is enough.

Do not immediately attempt:

```text
source ──implements──> target
source ──contradicts──> target
source ──supports──> target
...
```

unless the repository already contains enough explicit information to justify those distinctions.

The semantic richness can grow later.

---

## 26. What this gives you

The resulting system has several desirable properties.

### Humans can write naturally

Issue specifications remain Markdown.

People can write:

```markdown
I initially thought this belonged in the persistence layer,
but after implementing the first version I discovered that
the invariant actually belongs in the tracker domain.

The relevant code is [[Tracker::foo]].
```

No schema is required for the prose.

### Machines get stable references

The graph system resolves:

```text
[[Tracker::foo]]
```

to a canonical resource.

### Sections do not need manually maintained IDs

A section can be addressed through:

```text
document + heading
```

or another structural selector.

### Specifications can remain disposable

An issue specification can evolve, be rewritten, or eventually become obsolete.

### Durable behavior lives elsewhere

Behavioral tests can serve as the long-term executable specification.

### The graph can preserve history

The graph can retain relationships between issues, implementations, tests, decisions and historical artifacts.

### The repository does not become a database

The repository remains primarily a human-readable development environment.

---

## 27. The core design principle

The whole approach can be summarized as:

> **Structure the references, not the documents.**

Or more precisely:

> **Make semantic relationships explicit while allowing the artifacts containing those relationships to remain as flexible as their purpose requires.**

That leads to:

```text
                Human artifacts
                       │
       ┌───────────────┼────────────────┐
       │               │                │
    Markdown          Rust            Tests
       │               │                │
       └───────────────┼────────────────┘
                       │
                explicit references
                       │
                       ▼
                resource resolver
                       │
                       ▼
                 semantic graph
                       │
                       ▼
                knowledge database
```

The graph does not need to understand everything.

It only needs to understand **what was explicitly connected to what, where that connection came from, and eventually what kind of relationship it represents**.

That is a much more tractable problem than turning the entire repository into a formally structured representation.

---

## 28. Recommended direction for Torrust

For the Torrust Tracker repository, the direction I would take is:

### Phase 1 — Unified references

Define one repository-wide reference syntax.

Support it initially in:

- Markdown
- issue specs
- ADRs
- Rust comments
- documentation
- tests

Do not attempt to define the full semantic ontology.

### Phase 2 — Resource resolution

Build indexes for:

- files
- Markdown documents
- Markdown headings
- issues
- ADRs
- Rust symbols
- tests

Resolve textual references into canonical resources.

### Phase 3 — Evidence graph

Store:

```text
source
target
source location
reference text
artifact type
```

Initially, the relationship can simply be:

```text
references
```

### Phase 4 — Semantic relationships

Introduce richer relationships only where the repository demonstrates a need:

```text
implements
tests
motivated-by
depends-on
supersedes
contradicts
explains
```

### Phase 5 — Knowledge graph database

Only after the repository has accumulated enough real relationships should the graph be moved into or synchronized with a dedicated graph database.

The database then becomes a queryable projection of the repository rather than the repository becoming a database.

---

## 29. Final conclusion

The strongest part of this approach is that it separates three things that are often conflated:

```text
Human expression
        ≠
Machine addressability
        ≠
Formal specification
```

An issue specification can remain a flexible human artifact.

A Markdown section can be addressable without having a manually assigned ID.

A Rust method can be resolved through its syntax-tree identity.

A behavioral test can provide a durable executable specification.

And all of them can participate in the same knowledge graph.

The graph therefore becomes a **semantic layer over the repository**, rather than a reason to redesign the repository around a rigid data model.

This also fits the broader goal of building a system around project theory: instead of attempting to capture the complete theory in one formal representation, the system maintains explicit connections between the many artifacts through which that theory is expressed.

The most important practical rule is therefore:

> **Keep the issue specification disposable and human-friendly. Make the links explicit and machine-resolvable. Let durable behavior emerge in the executable acceptance tests. Build the knowledge graph from the links between these artifacts, not by trying to turn every artifact into a formal program.**

And, critically:

> **Do not design the complete theory of the graph before you have real graph data. Start with a tiny, unified linking mechanism and let the structure of the Torrust repository teach you what the graph needs to become.**
