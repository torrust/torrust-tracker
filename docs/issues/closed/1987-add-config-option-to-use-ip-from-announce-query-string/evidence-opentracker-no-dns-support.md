<!-- cspell:disable -->

# DNS Name Support in the `ip` Announce Parameter

## BEP 3 Specification

[BEP 3](https://www.bittorrent.org/beps/bep_0003.html) states about the `ip` GET parameter in the HTTP tracker announce request:

> **ip**: An optional parameter giving the IP (or dns name) which this peer is at. Generally used for the origin if it's on the same machine as the tracker.

## Finding: This Tracker Does NOT Support DNS Names in `ip`

The opentracker implementation does **not** support DNS names in the `ip` parameter. Only literal IPv4/IPv6 addresses are accepted, and even that only when explicitly enabled at compile time.

---

## Evidence

### 1. The `ip` parameter is gated behind a compile-time feature flag

**File:** `Makefile`, lines 24-25

```makefile
#FEATURES+=-DWANT_IP_FROM_QUERY_STRING
```

The feature is **commented out by default**. Without `-DWANT_IP_FROM_QUERY_STRING`, the `ip` parameter is not even recognized as a valid keyword.

**File:** `ot_http.c`, lines 497-503

```c
static ot_keywords keywords_announce[] = {
    {"port", 1}, {"left", 2}, {"event", 3}, {"numwant", 4},
    {"compact", 5}, {"compact6", 5}, {"info_hash", 6},
#ifdef WANT_IP_FROM_QUERY_STRING
    {"ip", 7},
#endif
#ifdef WANT_FULLLOG_NETWORKS
    {"lognet", 8},
#endif
    {"peer_id", 9}, {NULL, -3}};
```

The `{"ip", 7}` entry only exists in the keyword table when `WANT_IP_FROM_QUERY_STRING` is defined.

### 2. When enabled, the `ip` value is parsed with `scan_ip6()` — a literal IP parser only

**File:** `ot_http.c`, lines 607-614

```c
#ifdef WANT_IP_FROM_QUERY_STRING
    case 7: /* matched "ip" */
    {
      char *tmp_buf1 = ws->reply, *tmp_buf2 = ws->reply + 16;
      len           = scan_urlencoded_query(&read_ptr, tmp_buf2, SCAN_SEARCHPATH_VALUE);
      tmp_buf2[len] = 0;
      if ((len <= 0) || !scan_ip6(tmp_buf2, tmp_buf1))
        HTTPERROR_400_PARAM;
      OT_SETIP(&ws->peer, tmp_buf1);
    } break;
#endif
```

The value from the `ip` parameter is passed directly to `scan_ip6()`. This function comes from the [libowfat](http://www.fefe.de/libowfat/) library and is a pure string parser that only handles literal IPv6 address notation (including IPv4-mapped IPv6 addresses like `::ffff:192.0.2.1`). It does **not** perform DNS resolution.

### 3. No DNS resolution code exists anywhere in the codebase

A search across the entire repository for DNS-related functions returned zero results:

| Search Term     | Matches                                    |
| --------------- | ------------------------------------------ |
| `gethostbyname` | 0                                          |
| `getaddrinfo`   | 0                                          |
| `inet_pton`     | 0                                          |
| `inet_aton`     | 0                                          |
| `dns`           | 0 (only a false positive in `.git/hooks/`) |
| `resolve`       | 0                                          |

There is simply no code in this project that resolves hostnames to IP addresses.

### 4. The same pattern applies to the proxy/X-Forwarded-For path

**File:** `ot_http.c`, lines 521-528

```c
#ifdef WANT_IP_FROM_PROXY
  if (accesslist_is_blessed(cookie->ip, OT_PERMISSION_MAY_PROXY)) {
    ot_ip6 proxied_ip;
    char  *fwd = http_header(ws->request, ws->header_size, "x-forwarded-for");
    if (fwd && scan_ip6(fwd, proxied_ip)) {
      OT_SETIP(ws->peer, proxied_ip);
```

Even the alternative `WANT_IP_FROM_PROXY` path (which reads the peer IP from the `X-Forwarded-For` header) uses `scan_ip6()` and therefore also only accepts literal IP addresses, not DNS names.

---

## Summary

| Aspect                            | Status                                                                        |
| --------------------------------- | ----------------------------------------------------------------------------- |
| `ip` param recognized by default? | ❌ No — requires `-DWANT_IP_FROM_QUERY_STRING`                                |
| DNS names supported in `ip`?      | ❌ No — only literal IPv6/IPv4 addresses via `scan_ip6()`                     |
| Any DNS resolution in codebase?   | ❌ No — zero occurrences of `gethostbyname`, `getaddrinfo`, `inet_pton`, etc. |

The BEP 3 specification allows DNS names in the `ip` parameter, but this tracker implementation does not support them. To add DNS name support, one would need to:

1. Enable `WANT_IP_FROM_QUERY_STRING` at compile time.
2. Modify the `case 7` handler in `http_handle_announce()` to detect non-IP values and resolve them via `getaddrinfo()` before falling back to `scan_ip6()`.
