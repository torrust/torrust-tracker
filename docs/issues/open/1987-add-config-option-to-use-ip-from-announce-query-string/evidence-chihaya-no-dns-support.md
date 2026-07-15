<!-- cspell:disable -->

# BEP 3 DNS Name Support in the `ip` Parameter

**Date:** 2026-07-15
**Repository:** [chihaya/chihaya](https://github.com/chihaya/chihaya)
**Branch:** `main`

## The BEP 3 Requirement

[BEP 3](https://www.bittorrent.org/beps/bep_0003.html) defines the optional `ip` parameter in the HTTP tracker announce request as:

> _"An optional parameter giving the IP (or dns name) which this peer is at. Generally used for the origin if it's on the same machine as the tracker."_

This means the `ip` parameter should accept **both** IP addresses and DNS names (hostnames).

## Finding: Chihaya Does NOT Support DNS Names

Chihaya treats the `ip` parameter strictly as an IP address. DNS names are **not** supported. The value is always parsed with `net.ParseIP()`, which returns `nil` for any hostname.

## Evidence

### 1. Parsing — `frontend/http/parser.go`

The `requestedIP()` function resolves the peer's IP address. All paths call `net.ParseIP()`:

- **Line 152** — `"ip"` query param: [`net.ParseIP(ipstr)`](https://github.com/chihaya/chihaya/blob/main/frontend/http/parser.go#L152)
- **Line 155** — `"ipv4"` query param: [`net.ParseIP(ipstr)`](https://github.com/chihaya/chihaya/blob/main/frontend/http/parser.go#L155)
- **Line 158** — `"ipv6"` query param: [`net.ParseIP(ipstr)`](https://github.com/chihaya/chihaya/blob/main/frontend/http/parser.go#L158)
- **Line 163** — `RealIPHeader` (e.g. `X-Forwarded-For`): [`net.ParseIP(ip)`](https://github.com/chihaya/chihaya/blob/main/frontend/http/parser.go#L163)
- **Line 166** — `r.RemoteAddr` (TCP connection fallback): [`net.ParseIP(host)`](https://github.com/chihaya/chihaya/blob/main/frontend/http/parser.go#L166)

```go
// frontend/http/parser.go lines 148-167
func requestedIP(r *http.Request, p bittorrent.Params, opts ParseOptions) (ip net.IP, provided bool) {
    if opts.AllowIPSpoofing {
        if ipstr, ok := p.String("ip"); ok {
            return net.ParseIP(ipstr), true
        }

        if ipstr, ok := p.String("ipv4"); ok {
            return net.ParseIP(ipstr), true
        }

        if ipstr, ok := p.String("ipv6"); ok {
            return net.ParseIP(ipstr), true
        }
    }

    if opts.RealIPHeader != "" {
        if ip := r.Header.Get(opts.RealIPHeader); ip != "" {
            return net.ParseIP(ip), false
        }
    }

    host, _, _ := net.SplitHostPort(r.RemoteAddr)
    return net.ParseIP(host), false
}
```

If `net.ParseIP` returns `nil` (as it would for any DNS name), the request is rejected at **[line 112](https://github.com/chihaya/chihaya/blob/main/frontend/http/parser.go#L112)**:

```go
if request.IP.IP == nil {
    return nil, bittorrent.ClientError("failed to parse peer IP address")
}
```

### 2. Validation — `bittorrent/sanitize.go`

The `SanitizeAnnounce()` function performs a second validation in **[lines 28–37](https://github.com/chihaya/chihaya/blob/main/bittorrent/sanitize.go#L28-L37)**. The IP must be a valid IPv4 or IPv6 address; otherwise `ErrInvalidIP` is returned:

```go
if ip := r.IP.To4(); ip != nil {
    r.IP.IP = ip
    r.IP.AddressFamily = IPv4
} else if len(r.IP.IP) == net.IPv6len { // implies r.IP.To4() == nil
    r.IP.AddressFamily = IPv6
} else {
    return ErrInvalidIP
}
```

### 3. Data Structures — `bittorrent/bittorrent.go`

The `IP` type at **[line 210](https://github.com/chihaya/chihaya/blob/main/bittorrent/bittorrent.go#L210)** wraps `net.IP` — a raw byte representation of an IP address. It has no field to store a DNS name:

```go
type IP struct {
    net.IP
    AddressFamily
}
```

The `Peer` struct at **[line 230](https://github.com/chihaya/chihaya/blob/main/bittorrent/bittorrent.go#L230)** embeds this `IP` type:

```go
type Peer struct {
    ID   PeerID
    IP   IP
    Port uint16
}
```

### 4. No DNS Resolution in the Codebase

A search for `net.LookupHost`, `net.LookupIP`, or any DNS resolution function across the entire codebase returns **zero results**. There is no mechanism to resolve a hostname to an IP address.

## Impact

| Aspect                          | Current Behavior                                 |
| ------------------------------- | ------------------------------------------------ |
| `ip` param accepting DNS names  | ❌ No                                            |
| `net.ParseIP` on `ip` value     | ✅ Yes                                           |
| DNS resolution (`net.LookupIP`) | ❌ No                                            |
| Error returned for DNS names    | `ClientError("failed to parse peer IP address")` |

A DNS name like `"tracker.example.com"` would fail `net.ParseIP()` and be rejected with a client error before any further processing occurs.

## What Would Need to Change

To support DNS names as per BEP 3, the following areas would need modification:

1. **[`frontend/http/parser.go`](https://github.com/chihaya/chihaya/blob/main/frontend/http/parser.go)** — `requestedIP()`: detect when the value is a hostname (fails `net.ParseIP()` but is a non-empty string), then call `net.LookupIP()` to resolve it.
2. **[`bittorrent/bittorrent.go`](https://github.com/chihaya/chihaya/blob/main/bittorrent/bittorrent.go)** — `IP` struct: potentially store the original DNS name alongside the resolved IP.
3. **[`bittorrent/sanitize.go`](https://github.com/chihaya/chihaya/blob/main/bittorrent/sanitize.go)** — `SanitizeAnnounce()`: handle the case where the IP was resolved from a DNS name (the `AddressFamily` would be known after resolution).
