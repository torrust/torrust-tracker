# Snapshot: GitHub Copilot Memory for torrust/torrust-tracker

> **Status:** Immutable source snapshot. Do not edit the records below; record corrections and
> classifications in the analysis artifacts instead.
>
> **Issue contract:** [ISSUE.md](ISSUE.md)

## Provenance

- **Source:** GitHub repository settings page `Settings > Copilot > Memory` for
  `torrust/torrust-tracker` (repository memories).
- **Captured:** 2026-09-30, by the maintainer copying the rendered page HTML. GitHub Copilot
  converted the HTML to Markdown without changing any memory text.
- **Records:** 110, matching the page's "110 memories" count.
- **Order:** The page ranks memories "by usefulness": items near the top "have had stronger
  positive signals over time". IDs `G001`-`G110` follow that displayed order, so a lower ID means
  a higher rank on the capture date.
- **Hidden records:** The page states that "memories Copilot has learned not to use are hidden
  from this list". Hidden records are not in this snapshot.
- **Columns:** `Topic`, `Agent`, and `Model` are the three labels shown under each memory.
  Every record has the agent label `copilot-code-review`, so that column is omitted from the table.
- **Not captured:** creation dates, source PRs, and usage counts. The page does not show them.

## Conversion Notes

- Escaped characters only where Markdown would otherwise hide or change them: `\|` for literal
  pipes in table cells, `\<` for angle brackets outside code spans, and `\*` for literal
  asterisks outside code spans.
- Kept the original wording, spelling, and code spans, including the typographic apostrophe in
  `G109`.

## Privacy Review

Reviewed on 2026-09-30 before publication. No credentials, tokens, keys, email addresses, IP
addresses, hostnames other than `docs.rs`, local filesystem paths, or personal names were found.
Every record describes public repository code, documentation, or workflows. Nothing was
redacted.

## Records

| ID   | Memory | Topic | Model |
| ---- | ------ | ----- | ----- |
| G001 | Issue spec frontmatter uses `last-updated-utc: YYYY-MM-DD HH:MM` (UTC time required, not date-only). | documentation | gpt-5.2 |
| G002 | When moving an issue specification, update all live references to the former path, including semantic-links.related-artifacts and relative links; preserve old paths only in immutable historical records. | issue path moves | gpt-5.6-luna |
| G003 | Manual verification evidence links its issue specification with the `issue-spec` frontmatter key. | manual evidence metadata | gpt-5.6-luna |
| G004 | Issue-local agent review reports are append-only: add each completed review entry without modifying, reordering, or removing earlier entries. | review reports | gpt-5.6-luna |
| G005 | Folder-style issue specs must record manual verification commands, output, logs, and outcomes in issue-local manual-verification-evidence.md. | issue verification | gpt-5.6-luna |
| G006 | Issue `related-pr` metadata must be a PR number or null, not a URL. | issue frontmatter | gpt-5.6-luna |
| G007 | Independent review report entries are append-only: add new entries at the end and do not reorder, modify, or remove earlier entries. | review report history | gpt-5.6-luna |
| G008 | Current issue and EPIC frontmatter status values include `open` alongside draft, planned, in-progress, blocked, in-review, and done. | issue status schema | gpt-5.6-luna |
| G009 | Folder-based issue specs use docs/issues/{drafts\|open\|closed}/\<number>-slug/ISSUE.md with required frontmatter metadata. | issue specs | gpt-5.6-luna |
| G010 | In semantic-links.related-artifacts for long-lived docs, prefer `issue #NNNN` over issue-spec file paths since specs move between drafts/open/closed. | documentation | gpt-5.2 |
| G011 | Issue specs (docs/templates/ISSUE.md) include a "Commit Points" section mapping implementation-plan tasks to small, coherent, independently reviewable commits; create-issue skill requires drafting it. | documentation | gpt-5.2 |
| G012 | Supplementary issue-folder artifacts with frontmatter should point metadata back to the primary ISSUE.md; a self-referential spec-path does not identify the issue record. | supplementary metadata | gpt-5.6-luna |
| G013 | Keep `project-words.txt` entries in alphabetical order when adding new words. | documentation | gpt-5.2 |
| G014 | The hosted external-link workflow checks `docs/**/*.md` with Lychee while excluding only `docs/issues/closed/`; active issue documentation remains in scope. | external link workflow | gpt-5.6-luna |
| G015 | Independent review reports use docs/templates/AGENT-REVIEW-REPORTS.md; contract checks live in contrib/dev-tools/checks/tests/test-agent-review-report-contract.sh (run via bash). | documentation | gpt-5.2 |
| G016 | Issue spec frontmatter includes an `epic` field: set to parent EPIC number when established subissue, otherwise `null`. | documentation | gpt-5.2 |
| G017 | Durable documentation records use one folder per issue, EPIC, refactor plan, or PR-review audit with primary ISSUE.md, EPIC.md, REFACTOR-PLAN.md, or PR-REVIEW.md; companion artifacts stay beside it. | durable record layout | gpt-5.6-luna |
| G018 | Folder-style issue specs must record real manual verification scenarios and their actual commands, output, logs, and outcomes in issue-local manual-verification-evidence.md. | issue verification | gpt-5.6-luna |
| G019 | The maintained independent-review contract check is the Rust workspace package `agent-review-report-contract`; the former `contrib/dev-tools/checks/tests/test-agent-review-report-contract.sh` path is absent. | review contract check | gpt-5.6-luna |
| G020 | New normalized PR findings use immutable review-finding:pr-\<PR_NUMBER>-\<FINDING_ID> references with lowercase IDs; GitHub identifiers remain source metadata. | PR finding references | gpt-5.6-luna |
| G021 | Archiving an EPIC subissue requires updating its parent EPIC table to the closed spec path and DONE status. | issue archiving | gpt-5.6-sol |
| G022 | The vendored merge tool pushes directly to `githubmerge.host`/`githubmerge.repository`, not through a named Git remote. | merge workflow | gpt-5.6-sol |
| G023 | `stdout-result-data` commands emit exactly one JSON object on success and no stdout on failure; diagnostics are NDJSON on stderr. | CLI output | gpt-5.6-sol |
| G024 | GitHub Actions workflow files include a top-level `# skill-link: update-github-workflow-actions` marker. | workflow skill links | gpt-5.6-luna |
| G025 | Cleanup workflow commands use portable `find ... -exec basename {} \;` rather than GNU-only `find -printf`, preserving macOS/BSD usability. | cleanup portability | gpt-5.6-luna |
| G026 | Draft subissues of an EPIC use both an EPIC-number-prefixed folder and an `epic: <number>` frontmatter field, with the parent identified in the body. | draft EPIC parents | gpt-5.6-luna |
| G027 | Strict v1 issue and EPIC status values are draft, planned, in-progress, blocked, in-review, and done; `open` is invalid. | frontmatter status | gpt-5.6-luna |
| G028 | The frontmatter-validator binary is a no-stdout-result CLI: diagnostics are NDJSON on stderr and exit codes are 0 success, 1 validation/runtime failure, and 2 invalid invocation. | validator invocation | gpt-5.6-luna |
| G029 | Repository frontmatter is recognized only when the first line is exactly `---`; leading Markdown comments make metadata appear absent. | frontmatter parsing | gpt-5.6-luna |
| G030 | The repository frontmatter extractor recognizes YAML frontmatter only when `---` is the document's first line; a leading Markdown comment makes the metadata invisible to the validator. | frontmatter parsing | gpt-5.6-luna |
| G031 | Manual verification evidence files use frontmatter `last-updated-utc: YYYY-MM-DD HH:MM` and should be refreshed when evidence is appended. | verification metadata | gpt-5.6-luna |
| G032 | EPIC #1347 has separate aggregate and unit-only coverage tables; integration-only measurements are required in issue-local evidence rather than an EPIC table. | EPIC coverage tracking | gpt-5.6-luna |
| G033 | For a FIXED audit finding, cite the unique Conventional Commit subject; use a durable reply URL for NO_ACTION, SUPERSEDED, or FOLLOW_UP. | PR audit references | gpt-5.6-luna |
| G034 | PR audit records use a unique Conventional Commit subject as the resolution reference for FIXED findings and durable reply URLs for other dispositions. | PR audit references | gpt-5.6-luna |
| G035 | Issue-local verification evidence frontmatter uses a quoted `last-updated-utc` timestamp in `YYYY-MM-DD HH:MM` format. | evidence timestamps | gpt-5.6-luna |
| G036 | Frontmatter validator requires issue and EPIC `last-updated-utc` values to be double-quoted `YYYY-MM-DD HH:MM` UTC-minute strings. | frontmatter timestamps | gpt-5.6-luna |
| G037 | Bug specs must classify reproduction as Reproduced, Trigger only, or Infeasible and require a maintained regression test proven red before the fix. | bug workflow | gpt-5.6-luna |
| G038 | Canonical PR review audits live at docs/pr-reviews/pr-\<PR_NUMBER>-review/PR-REVIEW.md and cover findings from every reviewer. | PR audit records | gpt-5.6-luna |
| G039 | Markdown frontmatter must be enclosed by opening and closing `---` delimiters; top-of-file comments must not replace the opening delimiter. | frontmatter delimiters | gpt-5.6-luna |
| G040 | `github-review-threads` fetches up to 100 review threads with `resolvedBy` and `line`; `list`/`show` default to all threads, while `--unresolved-only` and `reply-status` remain action-only filters. | review-thread tool | gpt-5.6-luna |
| G041 | Quote complete `issue #NNNN` values in YAML frontmatter semantic-links because `#` starts a comment after whitespace. | frontmatter YAML quoting | gpt-5.6-luna |
| G042 | New strict issue specs use `schema-version: 1` and a quoted UTC-minute `last-updated-utc` timestamp. | issue frontmatter schema | gpt-5.6-luna |
| G043 | In frontmatter-validator, syntax.rs predicates are authoritative and each schema regex constant must describe the same language; schema tests pin the generated patterns to those constants. | schema syntax parity | gpt-5.6-luna |
| G044 | `stdout-result-data` binaries must emit exactly one JSON object plus newline on stdout; failures emit no stdout and JSON diagnostics on stderr. | CLI output contract | gpt-5.6-luna |
| G045 | Benchmark-report frontmatter uses `status: completed` for completed reports. | benchmark metadata | gpt-5.6-luna |
| G046 | `stdout-result-data` binaries emit exactly one JSON object on stdout; NDJSON diagnostics belong on stderr, and TTY stdout must be refused with exit code 2. | CLI output contract | gpt-5.6-luna |
| G047 | Adding an explicit Cargo workspace member requires corresponding Containerfile manifest COPY, target-stub, and nextest archive-list updates. | workspace members | gpt-5.6-luna |
| G048 | The tracker waits 10 seconds for JobManager shutdown, while token-aware REST draining uses a 90-second timeout. | shutdown deadlines | gpt-5.6-luna |
| G049 | Domain `ScrapeData` is keyed by info hash; UDP response encoding must preserve request order separately from the map. | UDP scrape ordering | gpt-5.6-luna |
| G050 | New PR-review audit records include semantic-links for process-pr-review and the matching skill-link marker. | PR audit links | gpt-5.6-luna |
| G051 | PR audit records under docs/pr-reviews/pr-\<PR>-review/PR-REVIEW.md include process-pr-review semantic-links frontmatter and a skill-link marker. | PR audit metadata | gpt-5.6-luna |
| G052 | Treat Dependabot GitHub Actions PRs as advisory; inventory all workflow references and follow the normal update, validation, and allowlist process. | workflow updates | gpt-5.6-luna |
| G053 | Package coverage uploads one artifact per matrix package and the summary job downloads `package-coverage-*` with `merge-multiple: true`. | coverage artifacts | gpt-5.6-luna |
| G054 | New strict v1 issue/EPIC specs include `schema-version: 1`; the validator applies strict required-field checks when that marker is present. | frontmatter schema | gpt-5.6-luna |
| G055 | Strict issue frontmatter parses `related-pr` as an optional positive pull-request number, not a URL. | issue frontmatter | gpt-5.6-luna |
| G056 | Bug issue specs must link Bug-Fix Process and Regression Test Strategy to fix-bug and require issue-local manual evidence. | bug issue specs | gpt-5.6-luna |
| G057 | The frontmatter validator rejects explicit null semantic-link sequences during extraction, while strict profile fields are optional vectors; schema projections must preserve this non-null-when-present rule. | frontmatter schema boundary | gpt-5.6-luna |
| G058 | JobManager owns only direct component futures; each component owns and joins or deliberately aborts its nested tasks before reporting a terminal outcome. | shutdown ownership | gpt-5.6-luna |
| G059 | The trusted PR coverage uploader is triggered by the exact Generate Coverage Report (PR) workflow name; rename both files together. | workflow coupling | gpt-5.6-luna |
| G060 | Folder-style implementation retrospectives require semantic-links YAML frontmatter before the title. | retrospective metadata | gpt-5.6-luna |
| G061 | The #2158 Clippy inventory Source location column preserves the original audit anchor; final line numbers are not maintained during remediation. | Clippy inventory | gpt-5.6-luna |
| G062 | Every new in-repo workspace crate must be added to Containerfile manifest/stub lists and corresponding .dockerignore exceptions. | workspace members | gpt-5.6-luna |
| G063 | Adding an explicit Cargo workspace member requires syncing Containerfile manifest/stub lists, all four nextest archive exclusion lists when developer-only, and .dockerignore exceptions. | workspace members | gpt-5.6-luna |
| G064 | Tracked SKILL.md files currently use both nested `metadata.semantic-links` and top-level `semantic-links`; frontmatter tooling must classify or preserve both placements. | skill frontmatter | gpt-5.6-luna |
| G065 | Docker E2E workflows must qualify e2e runner cargo commands with `-p torrust-tracker-e2e-tools`; the root package does not own these binaries. | E2E workflow commands | gpt-5.6-luna |
| G066 | Local Lychee checks use root lychee.toml, while the hosted external-link workflow explicitly uses .github/lychee-online.toml and scans docs/\*\*/\*.md. | Lychee configurations | gpt-5.6-luna |
| G067 | Immutable external snapshots use docs/external-snapshots/\<source>/\<version-or-revision>/ with unmodified source/license files and PROVENANCE.md recording revision, checksums, license, retrieval, and update policy. | external snapshots | gpt-5.6-luna |
| G068 | Markdown frontmatter uses space-indented YAML; tabs must not be used for indentation in semantic-links or related-artifacts blocks. | YAML frontmatter | gpt-5.6-luna |
| G069 | Adding an explicit workspace member requires synchronized Containerfile cargo-chef manifest/stub entries, .dockerignore inclusion, and nextest archive exclusions for developer-only crates. | workspace member containers | gpt-5.6-luna |
| G070 | Explicit workspace members require synchronized Containerfile recipe manifest/stub copies, .dockerignore exceptions, and nextest archive exclusions when developer-only. | workspace members | gpt-5.6-luna |
| G071 | Spec-only PRs use a `{issue-number}-{short-description}-spec` branch and reserve the base issue branch for implementation. | spec branches | gpt-5.6-luna |
| G072 | For a spec-only PR, use an `{issue-number}-{short-description}-spec` branch and set the issue frontmatter `branch:` to that same `-spec` name; reserve the base name for implementation. | spec-only branches | gpt-5.6-luna |
| G073 | For active issue specs, repository cleanup guidance accepts `open` as a pre-closure status even though the semantic schema example omits it. | issue status | gpt-5.6-luna |
| G074 | The repository's current skill-link validator is at `.github/skills/dev/environment-setup/run-tracker-locally/scripts/validate-skill-links.sh`; invoke it from the repository root with that path. | skill validation | gpt-5.6-luna |
| G075 | For a created issue, issue-local manual verification evidence should use `issue: #<number>`; `issue-spec` is reserved for draft specifications. | manual evidence metadata | gpt-5.6-luna |
| G076 | Issue and EPIC frontmatter status values include draft, open, planned, in-progress, blocked, in-review, and done. | issue status | gpt-5.6-luna |
| G077 | New PR-review audits live at docs/pr-reviews/pr-\<PR_NUMBER>-review.md; the PR author owns the record and duplicate legacy Copilot audits use a -copilot-suggestions-legacy.md suffix. | PR audit location | gpt-5.6-luna |
| G078 | The vendored merge tool creates pull/\<PR>/{base,head,merge,local-merge} branches and deletes them in a finally block after merge processing. | merge temp branches | gpt-5.6-luna |
| G079 | github-merge.py refuses to construct or sign a local merge when refs/pull/\<n>/merge first parent differs from the fetched target tip. | merge workflow | gpt-5.6-luna |
| G080 | After SI-5, activity metrics is a direct JobManager JoinSet component; UDP IP-ban cleanup is the sole remaining register_legacy periodic job. | job supervision | gpt-5.6-luna |
| G081 | Unspawned direct JobManager runner constructors returning `impl Future` with owned collaborators use explicit clippy expectations for `manual_async_fn` and `needless_pass_by_value`. | supervised runners | gpt-5.6-luna |
| G082 | EPIC package-coverage tables track aggregate/global and unit-only results separately; integration-only measurements remain issue-local evidence. | coverage evidence | gpt-5.6-luna |
| G083 | EPIC #1347 requires review after every test-producing increment and a maintainer stop after the final increment before final verification or a PR. | EPIC 1347 workflow | gpt-5.6-luna |
| G084 | Issue-local implementation retrospectives are created when reusable lessons, material design changes, or meaningful deviations warrant them; otherwise the spec records why none is needed. | retrospectives | gpt-5.6-luna |
| G085 | EPIC #1347 tracks aggregate and unit-only package coverage in its tables; integration-only measurements remain in issue-local evidence. | coverage tables | gpt-5.6-luna |
| G086 | GitHub Actions workflows in this repository use both `.yaml` and `.yml` extensions; maintenance scans must cover both. | workflow file extensions | gpt-5.6-luna |
| G087 | GitHub workflow files carry `# skill-link: update-github-workflow-actions` near top-level metadata. | workflow maintenance | gpt-5.6-luna |
| G088 | GitHub Actions update PRs use explicit versioned refs and include complete before/after organization-allowlist evidence in the PR body. | workflow updates | gpt-5.6-luna |
| G089 | Workspace packages inherit the shared docs.rs URL `https://docs.rs/crate/torrust-tracker/` through `documentation.workspace = true`. | docs.rs links | gpt-5.6-luna |
| G090 | Independent review report entries must be appended chronologically; earlier entries must not be modified, reordered, or removed. | review reports | gpt-5.6-luna |
| G091 | Issue specs that relate to ADRs should list those ADRs in semantic-links.related-artifacts, not only in body sections. | ADR semantic links | gpt-5.6-luna |
| G092 | update-dependencies uses timestamped branches for trivial updates and issue-number branches for breaking updates. | dependency branches | gpt-5.6-luna |
| G093 | The dependency-update skill uses a timestamped branch for trivial updates and an issue-number branch for breaking-change updates. | dependency branches | gpt-5.6-luna |
| G094 | The local `linter lychee` path uses root `lychee.toml` in offline mode, while the scheduled/manual External Link Check uses `.github/lychee-online.toml` explicitly. | Lychee configurations | gpt-5.6-luna |
| G095 | testing.yaml and container.yaml skip runs when all changed paths are Markdown files or project-words.txt via paths-ignore. | workflow path filters | gpt-5.6-luna |
| G096 | Testing and Container workflows skip runs when all changed files are Markdown or project-words.txt via paths-ignore. | CI path filters | gpt-5.6-luna |
| G097 | New or materially refactored tests must undergo a prose-first Arrange-Act-Assert comparison per write-unit-test skill; record the comparison in task evidence/plan before review+commit. | testing practices | gpt-5.2 |
| G098 | Config TOML [metadata].purpose is an enum that currently only allows "configuration" (lowercase) and defaults to that value. | configuration | gpt-5.2 |
| G099 | Tracked repository test code must be Rust; Python is allowed only for separately justified non-test external tooling (not test automation/fixtures/assertions). | testing | gpt-5.2 |
| G100 | Repo has multi-line crate-level Clippy allow attributes using `#![allow( ... clippy::... )]` (not single-line). | clippy allows | gpt-5.2 |
| G101 | Security analysis catalog entries in docs/security/analysis/production/ use the frontmatter key `cve-id` for CVEs and RustSec advisory IDs (not `finding-id`). | documentation | gpt-5.2 |
| G102 | JobManager supervises direct component futures via spawn(name, future) into a JoinSet; pre-spawned periodic JoinHandles use register_legacy(name, handle) and are still joined/escalated outside the JoinSet. | jobs | gpt-5.2 |
| G103 | Copilot PR review records use docs/copilot-pr-reviews/pr-\<PR_NUMBER>-copilot-suggestions.md with a reusable template and per-thread status table. | Copilot review records | gpt-5.6-luna |
| G104 | Before adding bespoke test helpers or refactoring generated tests, consult docs/testing/refactoring-patterns/README.md for repository-native patterns; prefer existing catalog patterns when they fit. | testing | gpt-5.2 |
| G105 | New issue specs must use status values draft, planned, in-progress, blocked, in-review, or done; `open` is not in the documented enum. | issue status | gpt-5.6-luna |
| G106 | AccessTokens is a type alias for HashMap\<String, SecretString> (unordered iteration). | configuration | gpt-5.2 |
| G107 | Pre-push checks intentionally exclude E2E tests; they run nightly fmt/check/doc plus the full stable workspace test suite, with E2E left to CI as merge authority. | testing | gpt-5.2 |
| G108 | Health-check integration tests use reqwest with Rustls; when ring and aws-lc-rs are enabled, install one CryptoProvider before parallel tests build clients. | Rustls test setup | gpt-5.6-luna |
| G109 | Git hook scripts pre-commit.sh and pre-push.sh mkdir -p $TORRUST_GIT_HOOKS_LOG_DIR (default /tmp) and abort if the dir isn’t writable. | git hooks | gpt-5.2 |
| G110 | project-words.txt is kept in case-insensitive alphabetical order; new words should be inserted in sorted position. | documentation | gpt-5.2 |
