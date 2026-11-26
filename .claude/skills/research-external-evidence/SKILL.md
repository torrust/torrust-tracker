---
name: research-external-evidence
description: "Gather evidence from other trackers' source, issue trackers, BEPs and docs to answer one question, and report facts with sources. Use when a claim about what other projects do needs verifying."
metadata:
  author: torrust
  version: "1.0"
  semantic-links:
    related-artifacts:
      - .claude/agents/researcher.md
---

# Researching External Evidence

Use this skill when a question about other projects needs evidence before a decision: what other
BitTorrent trackers (opentracker, chihaya and others) do for a protocol behavior or a BEP
requirement, what the de-facto standard is, which configuration options another tracker exposes, or
what its issue tracker says about a design decision. The session that researches follows it itself,
or through the [`researcher`](../../agents/researcher.md) agent, which preloads it.

You gather facts. You do not make implementation decisions and you do not write production code.
For questions about this repository's own code, search the repository (in Claude Code, the Explore
subagent does it), not this skill. When the caller wants the research kept as a document, it goes
under `docs/research/` as [`docs/research/AGENTS.md`](../../../docs/research/AGENTS.md) describes.

## Workflow

1. **Clarify the question.** State exactly what evidence is needed and from which external sources.
   When the question is ambiguous and the answer would change what you gather, ask, and do not guess.
2. **Plan the investigation.** Decide which repositories to clone, which queries to run and which
   documentation to read.
3. **Gather the evidence.**
   - Source code: make a shallow clone (`git clone --depth 1`), then use `grep`, `find` and
     `git log` to locate the code. Clone outside the workspace, for example in
     `/tmp/tracker-research/`; when `/tmp` is not available or the caller prefers a workspace-local
     path, use the git-ignored `.tmp/` directory. Never clone inside a tracked folder of the
     workspace.
   - Issues and discussions: use the search tools of the session when they are available, or the
     GitHub CLI (`gh search issues --repo <owner/repo> <keywords>`, `gh issue list`).
   - Documentation: fetch and read the BEPs, wiki pages and READMEs that bear on the question.
4. **Cross-reference.** Compare the evidence across sources and note where they agree and where they
   disagree. When the source code is ambiguous, say so instead of over-interpreting it.
5. **Report the findings** in the order of the next section. Delete the clones you made before
   you report, once the reply file (when there is one) is written, unless the caller needs them
   kept.

## Report

Return, in this order:

1. The research question, restated.
2. The sources consulted: repositories cloned, queries run, documents read.
3. The findings, for each source: what was found, with the file path and line number, the URL or
   the commit hash, and a direct quote or snippet where it helps.
4. A cross-project comparison, as a table or a summary of how each project handles the behavior.
5. The conclusion: what the evidence supports, with a confidence level.
6. The open questions: whatever the evidence did not resolve.

## Boundaries

- Do not create or change any file of the workspace: this is a read-only role. The exceptions are
  the reply file that the caller names (see the agent's instructions) and the clones you make in the
  git-ignored `.tmp/` directory.
- Do not recommend an implementation: report facts and let the caller decide.
- Do not guess or assume a behavior: every claim rests on evidence found in this session, with a
  source.
- Stay on the question: do not follow tangents that do not bear on it.
- Prefer shallow clones to keep the time and the disk use low.
