# Manual Verification — Issue #1985

**Date**: 2026-07-16  
**Branch**: `1985-rename-peer-addr-to-ip-in-http-announce-request`  
**Tracker**: local build (`./target/debug/torrust-tracker`, default dev config on `http://127.0.0.1:7070`)

---

## Setup

```bash
# Build
cargo build --bin torrust-tracker

# Clean DB and start tracker
rm -f ./storage/tracker/lib/database/sqlite3.db
RUST_LOG=info ./target/debug/torrust-tracker &

# Test values
BASE="http://127.0.0.1:7070"
INFO_HASH_ENC='%3b%24U%04%cf%5f%11%bb%db%e1%20%1c%eajk%f4Z%ee%1b%c0' # cspell:disable-line
PEER_ID='-RC3000-000000000001'
```

---

## M1 — Announce with `ip=<address>` (valid IP accepted)

**Command**:

```bash
curl -s "${BASE}/announce?info_hash=${INFO_HASH_ENC}&peer_id=${PEER_ID}&port=6881&uploaded=0&downloaded=0&left=0&event=started&compact=1&ip=2.137.87.41"
```

**Tracker log** (HTTP 200, announce processed):

```text
INFO request{...&ip=2.137.87.41 ...}: HTTP TRACKER: request ...
INFO request{...&ip=2.137.87.41 ...}: HTTP TRACKER: response ... status_code=200 OK
```

**Response**:

```text
d8:completei1e10:incompletei0e8:intervali120e12:min intervali120e5:peers0:6:peers60:e
```

**Result**: ✅ PASS — valid bencoded announce response returned; no parse error.

---

## M2 — Announce with old `peer_addr=<address>` (param ignored)

**Command**:

```bash
curl -s "${BASE}/announce?info_hash=${INFO_HASH_ENC}&peer_id=${PEER_ID}&port=6881&uploaded=0&downloaded=0&left=0&event=started&compact=1&peer_addr=2.137.87.41"
```

**Tracker log** (HTTP 200, `peer_addr=` visible in URI but tracker processes request normally):

```text
INFO request{...&peer_addr=2.137.87.41 ...}: HTTP TRACKER: request ...
INFO request{...&peer_addr=2.137.87.41 ...}: HTTP TRACKER: response ... status_code=200 OK
```

**Response**:

```text
d8:completei1e10:incompletei0e8:intervali120e12:min intervali120e5:peers0:6:peers60:e
```

**Result**: ✅ PASS — old `peer_addr=` parameter is silently ignored; no failure reason returned.

---

## M3 — Announce with `ip=hostname.example.com` (DNS name silently ignored)

**Command**:

```bash
curl -s "${BASE}/announce?info_hash=${INFO_HASH_ENC}&peer_id=${PEER_ID}&port=6881&uploaded=0&downloaded=0&left=0&event=started&compact=1&ip=hostname.example.com"
```

**Tracker log** (HTTP 200, DNS name visible in URI but tracker processes request normally):

```text
INFO request{...&ip=hostname.example.com ...}: HTTP TRACKER: request ...
INFO request{...&ip=hostname.example.com ...}: HTTP TRACKER: response ... status_code=200 OK
```

**Response**:

```text
d8:completei1e10:incompletei0e8:intervali120e12:min intervali120e5:peers0:6:peers60:e
```

**Result**: ✅ PASS — DNS name in `ip=` is silently dropped (field set to `None`); no failure reason returned; announce proceeds using connection IP.

---

## Summary

| ID  | Scenario                                                               | Result  |
| --- | ---------------------------------------------------------------------- | ------- |
| M1  | `ip=2.137.87.41` — valid IP accepted, normal announce response         | ✅ PASS |
| M2  | `peer_addr=2.137.87.41` — old param silently ignored, normal response  | ✅ PASS |
| M3  | `ip=hostname.example.com` — DNS name silently ignored, normal response | ✅ PASS |
