---
semantic-links:
  related-artifacts:
    - docs/packages.md
    - packages/AGENTS.md
    - docs/issues/open/1669-overhaul-packages/EPIC.md
    - docs/issues/open/1669-overhaul-packages/workspace-coupling-report-2026-10-06.md
---

# Torrust Tracker — Workspace Package Dependencies

Direct normal (non-dev, non-build) dependencies between workspace packages and on external
`torrust-*` crates, as declared in each `Cargo.toml`. Verified against
`cargo metadata --no-deps` on 2026-10-06; see the
[2026-10-06 coupling report](../../issues/open/1669-overhaul-packages/workspace-coupling-report-2026-10-06.md)
for the items each edge imports.

```mermaid
flowchart TB
    subgraph app["Application"]
        direction TB
        tracker["torrust-tracker<br/>(root crate)"]
    end

    subgraph servers["Servers"]
        direction TB
        axum-http["axum-http-server"]
        axum-rest["axum-rest-api-server"]
        axum-health["axum-health-check-api-server"]
        udp-srv["udp-server"]
        axum-base["axum-server"]
    end

    subgraph runtime-adapter["Runtime Adapter"]
        direction TB
        rest-adapter["rest-api-runtime-adapter"]
    end

    subgraph rest-application["REST API Application"]
        direction TB
        rest-app["rest-api-application"]
    end

    subgraph core["Core"]
        direction TB
        tracker-core["tracker-core"]
        http-core["http-core"]
        udp-core["udp-core"]
    end

    subgraph protocol["Protocols"]
        direction TB
        http-proto["http-protocol"]
        udp-proto["udp-protocol"]
        rest-proto["rest-api-protocol"]
    end

    subgraph domain["Domain / Shared"]
        direction TB
        swarm["swarm-coordination-registry"]
        config["configuration"]
        primitives["primitives"]
        events["events"]
    end

    subgraph client-tools["Client Tools"]
        direction TB
        client-lib["tracker-client-lib"]
        tracker-client["tracker-client<br/>(console)"]
        rest-client["rest-api-client"]
    end

    subgraph testing["Testing / Benchmarking"]
        direction TB
        test-helpers["test-helpers"]
        torrent-bench["torrent-repository-benchmarking"]
        persist-bench["persistence-benchmark"]
        e2e-tools["e2e-tools"]
    end

    subgraph external["External torrust-* crates"]
        direction TB
        clock["torrust-clock"]
        info-hash["torrust-info-hash"]
        located-err["torrust-located-error"]
        metrics["torrust-metrics"]
        net-prim["torrust-net-primitives"]
        peer-id["torrust-peer-id"]
        bencode["torrust-bencode"]
        server-lib["torrust-server-lib"]
    end

    %% App (composition root)
    tracker --> clock
    tracker --> server-lib
    tracker --> axum-health
    tracker --> axum-http
    tracker --> axum-rest
    tracker --> axum-base
    tracker --> config
    tracker --> tracker-core
    tracker --> events
    tracker --> http-core
    tracker --> primitives
    tracker --> rest-client
    tracker --> rest-proto
    tracker --> rest-adapter
    tracker --> swarm
    tracker --> udp-core
    tracker --> udp-srv

    %% Server dependencies
    axum-http --> clock
    axum-http --> info-hash
    axum-http --> net-prim
    axum-http --> server-lib
    axum-http --> axum-base
    axum-http --> config
    axum-http --> tracker-core
    axum-http --> http-core
    axum-http --> http-proto
    axum-http --> primitives
    axum-http --> swarm

    axum-rest --> clock
    axum-rest --> info-hash
    axum-rest --> metrics
    axum-rest --> net-prim
    axum-rest --> server-lib
    axum-rest --> axum-base
    axum-rest --> config
    axum-rest --> tracker-core
    axum-rest --> http-core
    axum-rest --> primitives
    axum-rest --> rest-app
    axum-rest --> rest-client
    axum-rest --> rest-proto
    axum-rest --> rest-adapter
    axum-rest --> swarm
    axum-rest --> udp-core
    axum-rest --> udp-srv

    axum-health --> net-prim
    axum-health --> server-lib
    axum-health --> axum-base
    axum-health --> config
    axum-health --> primitives

    axum-base --> located-err
    axum-base --> server-lib
    axum-base --> config

    udp-srv --> clock
    udp-srv --> info-hash
    udp-srv --> metrics
    udp-srv --> net-prim
    udp-srv --> peer-id
    udp-srv --> server-lib
    udp-srv --> client-lib
    udp-srv --> config
    udp-srv --> tracker-core
    udp-srv --> events
    udp-srv --> primitives
    udp-srv --> swarm
    udp-srv --> udp-core
    udp-srv --> udp-proto

    %% Core layer dependencies
    tracker-core --> clock
    tracker-core --> info-hash
    tracker-core --> located-err
    tracker-core --> metrics
    tracker-core --> config
    tracker-core --> events
    tracker-core --> primitives
    tracker-core --> swarm

    http-core --> clock
    http-core --> info-hash
    http-core --> metrics
    http-core --> net-prim
    http-core --> config
    http-core --> tracker-core
    http-core --> events
    http-core --> http-proto
    http-core --> primitives
    http-core --> swarm

    udp-core --> clock
    udp-core --> info-hash
    udp-core --> metrics
    udp-core --> net-prim
    udp-core --> config
    udp-core --> tracker-core
    udp-core --> events
    udp-core --> primitives
    udp-core --> swarm
    udp-core --> udp-proto

    rest-app --> info-hash
    rest-app --> primitives
    rest-app --> rest-proto

    rest-adapter --> info-hash
    rest-adapter --> metrics
    rest-adapter --> config
    rest-adapter --> tracker-core
    rest-adapter --> http-core
    rest-adapter --> primitives
    rest-adapter --> rest-app
    rest-adapter --> rest-proto
    rest-adapter --> swarm
    rest-adapter --> udp-core
    rest-adapter --> udp-srv

    %% Protocol layer
    http-proto --> bencode
    http-proto --> clock
    http-proto --> info-hash
    http-proto --> located-err
    http-proto --> peer-id

    udp-proto --> peer-id

    rest-proto --> metrics

    %% Domain layer (events has no torrust-* dependencies)
    swarm --> clock
    swarm --> info-hash
    swarm --> metrics
    swarm --> events
    swarm --> primitives

    config --> located-err
    config --> primitives

    primitives --> clock
    primitives --> info-hash
    primitives --> net-prim
    primitives --> peer-id

    %% Client tools
    client-lib --> located-err
    client-lib --> net-prim
    client-lib --> peer-id
    client-lib --> http-proto
    client-lib --> udp-proto

    tracker-client --> info-hash
    tracker-client --> peer-id
    tracker-client --> client-lib
    tracker-client --> http-proto
    tracker-client --> udp-proto

    rest-client --> rest-proto

    %% External crates with torrust-* dependencies
    server-lib --> net-prim

    %% Testing / Benchmarking
    test-helpers --> info-hash
    test-helpers --> peer-id
    test-helpers --> client-lib
    test-helpers --> config
    test-helpers --> http-proto
    test-helpers --> primitives
    test-helpers --> udp-proto

    torrent-bench --> clock
    torrent-bench --> info-hash
    torrent-bench --> primitives

    persist-bench --> info-hash
    persist-bench --> config
    persist-bench --> tracker-core
    persist-bench --> primitives

    e2e-tools --> tracker

    %% External crates styling
    classDef ext fill:#e1f5fe,stroke:#0288d1,stroke-dasharray: 5 5
    class clock,info-hash,located-err,metrics,net-prim,peer-id,bencode,server-lib ext

    %% Layer styling
    classDef app fill:#fff3e0,stroke:#ff9800
    class tracker app

    classDef srv fill:#e8f5e9,stroke:#4caf50
    class axum-http,axum-rest,axum-health,udp-srv,axum-base srv

    classDef core fill:#fce4ec,stroke:#e91e63
    class tracker-core,http-core,udp-core core

    classDef adapter fill:#ede7f6,stroke:#673ab7
    class rest-adapter adapter

    classDef application fill:#e3f2fd,stroke:#1976d2
    class rest-app application

    classDef proto fill:#f3e5f5,stroke:#9c27b0
    class http-proto,udp-proto,rest-proto proto

    classDef dom fill:#fff8e1,stroke:#ffc107
    class swarm,config,primitives,events dom

    classDef client fill:#e0f2f1,stroke:#009688
    class client-lib,tracker-client,rest-client client

    classDef test fill:#fafafa,stroke:#9e9e9e
    class test-helpers,torrent-bench,persist-bench,e2e-tools test
```
