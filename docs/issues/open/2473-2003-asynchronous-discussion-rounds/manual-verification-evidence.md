---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2473-2003-asynchronous-discussion-rounds/ISSUE.md
last-updated-utc: 2026-10-08 06:50
---

# Manual Verification Evidence

## Purpose

Record real, human-oriented verification of the completed behavior. This is
evidence from commands or interactions actually performed against the artifact;
do not invent commands, output, logs, or results.

## Environment and Prerequisites

- Date and time (UTC): 2026-10-08 06:46 to 06:50
- Artifact under test: branch `2473-asynchronous-discussion-rounds` at
  `docs(discussions): [#2473] hold discussions in attributed rounds` for V1 and the first part of
  V2; the V2 rerun used `docs(templates): [#2473] drop the shared participants row from the
  discussion template`.
- Operating system / environment: Linux 7.0.0-34-generic; `linter` 0.2.0 wrapping markdownlint
  0.46.0, cspell 10.0.1, and lychee 0.24.2 (offline, `include_fragments = "full"`).
- Prerequisites and setup performed: none beyond the branch checkout. The scratch discussion was
  created under `docs/discussions/2473-scratch-rounds-check/20261008-scratch-check/README.md` so
  the linters would scan it, and was never staged or committed.

## Verification Processes

### V1 - Open a Discussion from the Template (M1)

- Goal: following only `docs/discussions/AGENTS.md`, open a discussion from the template with two
  topics, and check that the linters accept it.
- Initial state: no scratch folder; template at `docs/templates/DISCUSSION.md`.
- Status: `DONE`

#### Steps Performed

1. Read `docs/discussions/AGENTS.md`: Layout gives the folder form
   `<issue-number>-<slug>/<YYYYMMDD>-<topic>/README.md` and links the template; Rounds describes the
   opening round.
2. `mkdir -p docs/discussions/2473-scratch-rounds-check/20261008-scratch-check`
3. `cp docs/templates/DISCUSSION.md docs/discussions/2473-scratch-rounds-check/20261008-scratch-check/README.md`
4. As the fictional opening author "Ana Example", replaced the placeholders, deleted the template
   comments, and wrote two topics (`Q1`, `Q2`), each with one position entry for PR #9001.
5. Ran `linter markdown`, `linter cspell`, and `linter lychee`. The scratch folder was kept for V2.

#### Observed Result

```text
linter markdown exit=0
linter cspell exit=0
linter lychee exit=0
linter 0.2.0
```

#### Conclusion

Met. The guide and the template's own comments were enough to open the discussion; no other
source was needed.

### V2 - Simulate a Contribution and a Decision (M2)

- Goal: add a second participant's entries and a decision-round Outcome, check that a reader can
  answer "who held which position on `Q1`, and in which PR?" from the `Q1` section alone, and that
  no participant edits another's entry.
- Initial state: the V1 scratch discussion.
- Status: `DONE` after one failure, a template fix, and a rerun.

#### Steps Performed

1. Contribution round as the fictional "Bo Second" (PR #9002): appended one entry under `Q1` and
   one under `Q2`, the `Q1` entry stating AI assistance. Following the template at that time, also
   added Bo Second to the shared `Participants` row.
2. Compared the file before and after the round with `diff`.
3. Decision round as the fictional decision owner "Dee Owner": set Status to `Decided` and wrote an
   Outcome citing Bo Second's `Q1` entry and Ana Example's `Q2` entry by heading anchor.
4. Ran the three linters.
5. Negative control: changed one Outcome anchor to the wrong fragment
   `#q1---bo-second-2026-10-09`, ran `linter lychee`, then restored the anchor and reran it.
6. Gave an explore agent that had not written the document only the `Q1` section, with the
   question "Who held which position on Q1, and in which pull request?", and no file or tool
   access.
7. Removed the `Participants` row from the template (see Failures and Follow-up), rebuilt the
   scratch file from the pre-round state without that row, repeated step 1 without the row edit,
   compared with `diff`, and reran the linters.
8. Saved the final scratch file to `.tmp/m2-final-scratch.md` (copied verbatim below) and deleted
   `docs/discussions/2473-scratch-rounds-check/`. The deletion came before this evidence was
   written, not after it as M2 says; every output was captured first.

#### Observed Result

First contribution round (step 2): `diff` showed one changed line, the shared `Participants` row,
besides the added entries:

```text
< | Participants   | Ana Example (`ana-example`)                                 |
```

Decision round and linters (steps 3 and 4):

```text
12c12
< | Status         | Open for rounds                                             |
---
> | Status         | Decided                                                     |
61c61,64
< Pending: no decision recorded.
---
> - **Q1: rejected.** The widget keeps its name, because of the type-name collision in
>   [Bo Second's entry](#q1---bo-second-bo-second-2026-10-09-pr-9002). Recorded in issue #2473.
> - **Q2: not needed** once Q1 is rejected; see
>   [Ana Example's entry](#q2---ana-example-ana-example-2026-10-08-pr-9001).
linter markdown exit=0
linter cspell exit=0
linter lychee exit=0
```

Negative control (step 5):

```text
negative control lychee exit=1
[ERROR] file:///…/docs/discussions/2473-scratch-rounds-check/20261008-scratch-check/README.md#q1---bo-second-2026-10-09 (at 62:3) | Cannot find fragment
restored lychee exit=0
```

Independent reader's answer (step 6), verbatim:

```text
Ana Example held “Yes” on Q1 in PR #9001.
Bo Second held “No” in PR #9002.
Bo’s position was AI-assisted during drafting, then reviewed and adopted by Bo.
The attribution is clear; only the AI tool/model are unspecified placeholders.
```

Rerun of the contribution round after the template fix (step 7):

```text
removed or changed lines: 0
added lines: 9
linter markdown exit=0
linter cspell exit=0
linter lychee exit=0
```

Final scratch discussion (step 8):

````markdown
---
semantic-links:
  related-artifacts:
    - "issue #2473"
    - docs/discussions/AGENTS.md
---

# Scratch Check: Naming the Example Widget

| Field          | Value                                                       |
| -------------- | ----------------------------------------------------------- |
| Status         | Open for rounds                                             |
| Started        | 2026-10-08                                                  |
| Opened by      | Ana Example (`ana-example`)                                 |
| Decision owner | Dee Owner (`dee-owner`), owner of issue #2473               |
| Informs        | Issue #2473 - Run design discussions as asynchronous rounds |
| Scope          | A scratch question used only to verify the round model      |
| AI assistance  | None                                                        |

## Context

This scratch discussion exists only to verify the discussion template and is never committed.

Nothing here changes a specification or a rule. A conclusion takes effect only when the decision
owner records it in the Outcome and in the canonical document it links.

## Topics

### Q1 - Should the widget be called "gadget"?

Whether to rename the example widget. Added by Ana Example.

#### Positions

##### Q1 - Ana Example (`ana-example`), 2026-10-08, PR #9001

Yes: "gadget" is shorter and already used in the user guide.

##### Q1 - Bo Second (`bo-second`), 2026-10-09, PR #9002

No: "gadget" collides with an existing type name. Drafted with an AI assistant (example tool,
example model); reviewed and adopted by Bo Second.

### Q2 - Should the rename keep an alias?

Whether the old name stays as an alias for one release. Added by Ana Example.

#### Positions

##### Q2 - Ana Example (`ana-example`), 2026-10-08, PR #9001

Yes, for one release, so existing configuration keeps working.

##### Q2 - Bo Second (`bo-second`), 2026-10-09, PR #9002

Agree with an alias, if the rename happens at all.

## Outcome

Pending: no decision recorded.
````

#### Conclusion

Met after the fix. The reader answered from the `Q1` section alone, the Outcome's citations are
checked by lychee (the negative control fails on a wrong anchor), and a contribution round now only
adds lines. The final scratch file above is the rerun state, before its decision round.

## Failures and Follow-up

- V2, first attempt: the template's `Participants` row asked each participant to add themselves.
  A contribution round therefore changed a shared line, so concurrent rounds would conflict and a
  round could not be append-only. Fixed in `docs(templates): [#2473] drop the shared participants
  row from the discussion template`, since entry headings already name every participant. Step 7
  reran the contribution round: 0 changed lines.
