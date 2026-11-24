---
semantic-links:
  skill-links:
    - create-adr
  related-artifacts:
    - .claude/skills/create-adr/SKILL.md
    - docs/adrs/20260519000000_define_global_cli_output_contract.md
    - docs/adrs/20261106102632_scope_the_cli_output_contract_and_keep_exit_codes_to_zero_one_two.md
    - docs/issues/open/2904-2003-migrate-custom-agents-to-claude-code/ISSUE.md
---

<!-- skill-link: create-adr -->

# Let Programs Follow the Protocol of the Host That Calls Them

## Scope

Repository-level decision: it changes how the global CLI output contract applies to every program in
the binary table, so it lives in the root collection, `docs/adrs/`. It amends
[Define the Global CLI Output Contract](20260519000000_define_global_cli_output_contract.md), which
the earlier amendment
[Scope the CLI Output Contract and Keep Exit Codes to 0, 1 and 2](20261106102632_scope_the_cli_output_contract_and_keep_exit_codes_to_zero_one_two.md)
had already amended.

## Description

The contract fixes how a program talks to the scripts, CI jobs and agents that call it: the
caller chooses the arguments, reads one JSON object on stdout, reads NDJSON diagnostics on stderr,
and reads the exit code as `0` success, `1` runtime failure, `2` usage error. It was written for
programs whose caller is a person or a script we control.

Some programs are not called that way. A host application, which we do not control, defines the
conversation, and the program is a plug-in of that host. The first such program is
`agent-reply-guard` (issue #2904), the stop hook of the `task-reviewer` agent, which the agent also
declares as a hook before the tool that hands its report back. Claude Code calls it and defines the
protocol:

- the input is a JSON object on stdin, with no arguments;
- exit code `2` means "refuse: keep the agent working", and the host feeds stderr to the agent as
  the reason;
- the reason must be plain text, because the agent reads it as an instruction;
- on exit `0` the host reads a JSON object on stdout, for example `systemMessage`;
- any other exit code is a non-blocking error: the host ignores the hook and the agent stops.

The contract and this protocol disagree on three points: exit `2` is a refusal and not a usage
error, stderr carries plain text and not NDJSON, and the input is stdin and not arguments. The
program cannot satisfy both, because the host does not change its protocol for us. Without a rule,
each such program would break the contract silently, or would be made to look compliant while not
working.

## Agreement

### 1. A third class: `external-protocol`

A program is `external-protocol` when a host application that the repository does not control
defines its input, its output and the meaning of its exit codes, and the program exists to be
called by that host. A hook, a plug-in or a callback of a tool we use is such a program. A program
that merely can be called by a third party, such as the tracker daemon, is not.

The class is defined by what the program is, not by which host it serves, so a later adapter (a git
hook, an editor integration) needs no new amendment of this ADR. It is not open-ended: membership
is a row in the binary table, so each adapter is named, reviewed in a diff and removable.

### 2. What the class changes, and what it does not

For an `external-protocol` program, the host's documented protocol replaces these parts of the
contract: the input, the content of stdout and stderr, and the meaning of the exit codes that the
protocol defines. Nothing else is relaxed:

- Where the protocol leaves a choice free, the contract applies. For `agent-reply-guard`: its own
  failures (unreadable input, an unreadable transcript) are errors of the program and not
  refusals, so they exit `1`, never `2`, and nothing it writes can keep an agent working by
  accident.
- The program's crate documentation names the protocol and links the host's documentation, so the
  difference from the contract is explained where the code is.
- The program is tested at its command boundary against the protocol as observed, and the
  observation is recorded (for the guard, scenarios M7 and M8 of issue #2904).

### 3. The binary table

The table gets no new column. An `external-protocol` row has `Class` set to `external-protocol`, and
its `Notes` cell starts with the protocol. `agent-reply-guard` gets the first row: Claude Code
`Stop` hook (run as `SubagentStop` in an agent's frontmatter).

No code check enforces the table today, so membership in this class is enforced by review.

## Alternatives Considered

**A concrete list of the exceptions in the ADR text.** Rejected. The binary table already lists
every program and its class, so the list would be a second copy that drifts. Its advantage is that
the exception is easy to find in the text; the table row gives the same visibility in the place a
reader already looks.

**A rule that names the host: "Claude Code hooks follow Claude Code".** Rejected as too narrow. It
is the easiest rule to review, but the next adapter, for another host, would need another
amendment, and the amendments would repeat each other.

**A fully generic rule with no table row: "programs follow their host's protocol".** Rejected as
open-ended. Any program could claim the exception, and nothing would show in a diff that one had.
Its advantage is that it needs no maintenance.

**A separate inventory of exceptions.** Rejected. It is a second source of truth beside the binary
table (policy 6 of `AGENTS.md`), and a reader of the table would not know it exists.

**Conform as far as possible: NDJSON on stderr and no stdout message.** Rejected. It cannot
conform fully, because the refusal must still exit `2`. The agent would read JSON lines where it
needs a sentence, and the "limit reached" notice shown to the user through `systemMessage` would be
lost. It keeps the ADR text untouched, which is its only advantage.

**Edit the original ADR in place.** Rejected for the reason the earlier amendment gave: an ADR
records a decision at a date, and this one came later. The original gets a short pointer.

## Consequences

### Positive

- A program that must follow a host's protocol can do so, with a recorded reason, and the agent that
  reads its refusal gets a sentence it can act on.
- The exception is visible in the binary table and limited by two conditions, so it cannot spread
  unreviewed.
- The contract keeps its meaning for every other program.

### Negative

- The contract is no longer one rule for every program: a reader must know the third class exists.
- A host that changes its protocol can break the adapter silently. The scenarios recorded as
  evidence and the boundary tests reduce this; they do not remove it.
- Membership is enforced by review, not by a check.

## Date

2026-11-24

## References

- Issue: [#2904](https://github.com/torrust/torrust-tracker/issues/2904), spec
  `docs/issues/open/2904-2003-migrate-custom-agents-to-claude-code/ISSUE.md` (decision D17)
- Amended ADR: [Define the Global CLI Output Contract](20260519000000_define_global_cli_output_contract.md)
- Earlier amendment:
  [Scope the CLI Output Contract and Keep Exit Codes to 0, 1 and 2](20261106102632_scope_the_cli_output_contract_and_keep_exit_codes_to_zero_one_two.md)
- Claude Code hooks reference: <https://code.claude.com/docs/en/hooks>
