# Research: `IPV6_V6ONLY` Defaults and Dual-Stack Portability

> Related to [#1671](https://github.com/torrust/torrust-tracker/issues/1671) — IPv4/IPv6 client metrics.

## Motivation

The tracker's experiment confirmed that setting `IPV6_V6ONLY=1` on Linux (with
`net.ipv6.bindv6only = 0`) allows separate IPv4/IPv6 sockets on the same port.
But the design of a permanent config option depends on understanding how this
works across platforms.

## Platform Defaults

| OS      | `IPV6_V6ONLY` default | Dual-stack by default? | Notes                                                                                                                                  |
| ------- | --------------------- | ---------------------- | -------------------------------------------------------------------------------------------------------------------------------------- |
| Linux   | `0` (off)             | ✅ Yes                 | Controlled by `net.ipv6.bindv6only` sysctl. Most distros keep `0`.                                                                     |
| Windows | `1` (on)              | ❌ No. IPv6-only       | Since Vista. Must explicitly `setsockopt` with `IPV6_V6ONLY=0` for dual-stack.                                                         |
| macOS   | `1` (on)              | ❌ No. IPv6-only       | Darwin/XNU defaults to IPv6-only.                                                                                                      |
| FreeBSD | `1` (on)              | ❌ No. IPv6-only       | Similar to other BSDs.                                                                                                                 |
| OpenBSD | `1` (forced)          | ❌ No, impossible      | Does **not support** IPv4-mapped addresses at all. `IPV6_V6ONLY` is effectively forced to `1` regardless of what the application sets. |
| Solaris | `1` (on)              | ❌ No. IPv6-only       | Same as other non-Linux Unix.                                                                                                          |

**Key takeaway**: Linux is the **only** major OS that defaults to dual-stack
(`IPV6_V6ONLY=0`). Every other platform is IPv6-only by default.

## Can we enable dual-stack at runtime if the OS has `net.ipv6.bindv6only = 1`?

**Yes**, easily. `net.ipv6.bindv6only` is a **system-wide sysctl** that sets the
default for all IPv6 sockets. But an application can override it per-socket by
calling `setsockopt(sock, IPPROTO_IPV6, IPV6_V6ONLY, &zero, sizeof(zero))` (i.e.
`set_only_v6(false)` in `socket2` terms) **before** `bind()`.

So the runtime control works both ways:

- `IPV6_V6ONLY=1` on Linux (dual-stack by default) → separate sockets ✅
- `IPV6_V6ONLY=0` on macOS/Windows/BSD (IPv6-only by default) → dual-stack socket ✅

The `socket2` API makes this uniform regardless of the OS default.

## Can we enable dual-stack at runtime on OpenBSD?

**No.** OpenBSD does not support IPv4-mapped IPv6 addresses at all. The kernel
rejects `IPV6_V6ONLY=0`. On OpenBSD, an IPv6 socket is always IPv6-only.

## Design Implications

### Option A: Always set `IPV6_V6ONLY=1` (IPv6-only sockets, separate binds required)

- **Linux**: Works. User must configure both `0.0.0.0:<port>` and `[::]:<port>`.
- **Windows/macOS/BSD**: Works (already the default, code is a no-op).
- **OpenBSD**: Works (already forced, code is a no-op).
- **Breakage**: Existing configs that only bind `[::]:<port>` will **lose IPv4
  support** on Linux. Operators must add explicit `0.0.0.0:<port>` entries.
- **Consistency**: Same behaviour on all platforms.

### Option B: Config toggle (default: dual-stack, opt-in: separate sockets)

- **No breakage** for existing users (default preserves current behaviour).
- Config toggle only works on platforms that support it (Linux, Windows, macOS,
  FreeBSD). On OpenBSD, forcing `IPV6_V6ONLY=0` is a runtime error.
- OS-dependent features are not unprecedented (e.g., `io_uring` is Linux-only),
  but they add maintenance burden.

### Option C: Always set `IPV6_V6ONLY=1` unconditionally (no config toggle)

- Consistent behaviour everywhere.
- Breaking change for Linux users who bind only `[::]:<port>`.
- Mitigation: release notes + migration guide in changelog.

## Recommendation

**Option B seems safest**: a config option (e.g.
`udp_tracker.ipv6_v6only` / `http_tracker.ipv6_v6only`) defaulting to `false`
(preserving current dual-stack behaviour). Operators who want separate sockets
can opt in. The option is documented as Linux/macOS/Windows-only; on OpenBSD the
application logs a warning and ignores the setting.

That said, Option C (always-on) has appeal for simplicity and cross-platform
consistency, but the breaking change needs careful handling.

## References

- [Biriukov: Dual-Stack Applications — IPV6_V6ONLY](https://biriukov.dev/docs/resolver-dual-stack-application/6-dual-stack-applications/#-ipv6_v6only-socket-option)
- [Microsoft: Dual-Stack Sockets for IPv6 Winsock Applications](https://learn.microsoft.com/en-us/windows/win32/winsock/dual-stack-sockets)
- [StackOverflow: Dual stack with one socket](https://stackoverflow.com/questions/22075363/dual-stack-with-one-socket)
- [Nginx listen directive — ipv6only](https://nginx.org/en/docs/http/ngx_http_core_module.html#listen)
- [RFC 3493 §3.7 — Compatibility with IPv4 Nodes](https://datatracker.ietf.org/doc/html/rfc3493#section-3.7)
- [RFC 4291 §2.5.5.2 — IPv4-mapped IPv6 addresses](https://datatracker.ietf.org/doc/html/rfc4291#section-2.5.5.2)
- [FreeBSD forums: Creating a IPv4/IPv6 socket in C](https://forums.freebsd.org/threads/creating-a-ipv4-ipv6-socket-in-c.92530/)
- OneUptime: [Dual-stack sockets & IPV6_V6ONLY](https://github.com/oneuptime/blog/tree/master/posts/2026-03-20-dual-stack-sockets-ipv6-v6only)
- OneUptime: [Prefer IPv4/IPv6 config](https://oneuptime.com/blog/post/2026-03-20-prefer-ipv4-ipv6-config/view)
- [ForestVPN: Disable IPv6 on Windows/macOS/Linux](https://forestvpn.com/en/blog/networking/disable-ipv6-windows-macos-linux/)
- [StackOverflow: What was the motivation for adding IPV6_V6ONLY?](https://stackoverflow.com/questions/2693709/what-was-the-motivation-for-adding-the-ipv6-v6only-flag)
