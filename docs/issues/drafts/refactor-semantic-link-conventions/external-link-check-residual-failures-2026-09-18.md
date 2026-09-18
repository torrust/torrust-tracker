---
source-run: https://github.com/torrust/torrust-tracker/actions/runs/35315382956
source-revision: e6dd8918bf6964794882f60486f2da3e296cbb13
online-config: .github/lychee-online.toml
collected-utc: 2026-09-18
---

# External Link Check Residual Failures

This artifact preserves the last `lychee-external-link-report` produced while issue
[#2185](https://github.com/torrust/torrust-tracker/issues/2185) was open. It is input for this
EPIC, not a decision. The failures listed here are the cases that the operational triage in #2185
could not resolve by repairing a stale reference or by adding a narrowly scoped exclusion to
[`.github/lychee-online.toml`](../../../../.github/lychee-online.toml).

The report comes from [External Link Check run 35315382956](https://github.com/torrust/torrust-tracker/actions/runs/35315382956),
which ran on merged `develop` revision `e6dd8918` after PR #2255 merged. The 782-byte artifact
expires on 2026-10-02; the report is reproduced verbatim below so the evidence outlives it.

## Why These Cases Remain

| Case | Documents | Observed diagnostic | Root cause established in #2185 |
| ---- | --------- | ------------------- | ------------------------------- |
| Medium articles | `.github/skills/dev/testing/write-unit-test/SKILL.md` | `403 Forbidden` | Medium returns `403` to both a Lychee-like and a browser user agent; this is bot protection, not stale content. |
| Stack Overflow answer permalink | `docs/containers.md` | `403 Forbidden` | The short `/a/<answer>/<user>` permalink redirects to the full question URL, which then also returns `403` to automated clients. The answer still exists. |
| FSF home page | `README.md`, `console/tracker-client/README.md`, `packages/rest-api-client/README.md`, `packages/tracker-client/README.md` | TLS `HandshakeFailure` and cached errors | `www.fsf.org` returns HTTP 200 via `curl` and `openssl`, but only offers finite-field `DHE` cipher suites. Lychee uses rustls, which supports `ECDHE` only, so the checker can never complete the handshake. This is a checker limitation, not a dead link. |
| GNU licenses page | Same four license footers as FSF | `Request timed out` | First observed in this run. Under the rerun-first policy in `docs/testing.md`, a single timeout is a transient candidate until a second run confirms it. |

These links are all AGPL license boilerplate or background reading. None of them is a repository-owned
reference that the project can repair, and each candidate fix (replacing citations, excluding hosts,
or disabling TLS strictness) is a policy decision rather than a cleanup step. Issue #2185 therefore
stopped here and handed the question to this EPIC.

## Verbatim Report

The report is quoted as a fenced block so the online check does not re-check these URLs from this
artifact.

```text
# Summary

| Status         | Count |
|----------------|-------|
| 🔍 Total       | 2224  |
| 🔗 Unique      | 1537  |
| ✅ Successful  | 1327  |
| ⏳ Timeouts    | 4     |
| 🔀 Redirected  | 27    |
| 👻 Excluded    | 886   |
| ❓ Unknown     | 0     |
| 🚫 Errors      | 7     |
| ⛔ Unsupported | 0     |

## Errors per input

### Errors in .github/skills/dev/testing/write-unit-test/SKILL.md

* [403] <https://medium.com/@kentbeck_7670/programmer-test-principles-d01c064d7934> (at 41:1) | Rejected status code: 403 Forbidden
* [403] <https://medium.com/@kentbeck_7670/test-desiderata-94150638a4b3> (at 40:1) | Rejected status code: 403 Forbidden

### Errors in console/tracker-client/README.md

* [ERROR] <https://www.fsf.org/> (at 190:162) | Network error: received fatal alert: HandshakeFailure

### Errors in docs/containers.md

* [403] <https://stackoverflow.com/a/56768087/3012842> (at 571:5) | Rejected status code: 403 Forbidden

### Errors in packages/rest-api-client/README.md

* [ERROR] <https://www.fsf.org/> (at 9:162) | Error (cached)

### Errors in packages/tracker-client/README.md

* [ERROR] <https://www.fsf.org/> (at 11:162) | Error (cached)

### Errors in README.md

* [ERROR] <https://www.fsf.org/> (at 226:162) | Error (cached)

## Timeouts per input

### Timeouts in console/tracker-client/README.md

* [TIMEOUT] <https://www.gnu.org/licenses/> (at 194:114) | Request timed out

### Timeouts in packages/rest-api-client/README.md

* [TIMEOUT] <https://www.gnu.org/licenses/> (at 13:114) | Request timed out

### Timeouts in packages/tracker-client/README.md

* [TIMEOUT] <https://www.gnu.org/licenses/> (at 15:114) | Request timed out

### Timeouts in README.md

* [TIMEOUT] <https://www.gnu.org/licenses/> (at 230:114) | Request timed out
```
