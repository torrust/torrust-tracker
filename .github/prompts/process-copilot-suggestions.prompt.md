---
name: "Process Copilot Suggestions"
description: "Review, address, reply to, and resolve Copilot suggestions on the current or specified pull request"
argument-hint: "Optional PR number; defaults to the active pull request"
agent: "Copilot Suggestions Handler"
---

Process Copilot's review suggestions on this repository's pull request by strictly following the canonical [process Copilot suggestions skill](../skills/dev/pr-reviews/process-copilot-suggestions/SKILL.md) and all applicable repository instructions.

Target pull request: ${input:PR number (leave empty for the active PR):}

If no PR number is supplied, identify the active pull request. Process **only Copilot-authored unresolved review threads**; do not modify human reviewer threads.

Use the full auditable workflow:

1. Create or update `docs/pr-reviews/pr-<PR_NUMBER>-copilot-suggestions.md` from the tracker template.
2. Fetch every unresolved review thread with the repository helper scripts and record it in the tracker.
3. Handle one thread at a time: decide `action` or `no-action`; for an action, make the smallest correct fix, validate it, create a GPG-signed commit through the Committer agent, and push it.
4. Always reply with the outcome before resolving that same thread. Use the repository's atomic reply-and-resolve helper; record its reply URL and final status in the tracker.
5. After every push, refetch review threads and process any newly created Copilot threads.
6. Stop only after no Copilot-authored unresolved threads remain. Complete and GPG-sign the tracker-documentation commit, then report the decisions, commits, validation, reply URLs, and any deliberately declined suggestions.

Do not resolve a thread without a reply. Do not batch-resolve threads. Do not expand a suggestion into an unrelated refactor or feature; explain and decline it or record a follow-up when appropriate. Do not push a fix without the required pre-commit gate.
