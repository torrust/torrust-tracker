---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2904-2003-migrate-custom-agents-to-claude-code/ISSUE.md
last-updated-utc: "2026-11-19 16:57"
---

# Manual Verification Evidence

## Purpose

Record real, human-oriented verification of the completed behavior. This is
evidence from commands or interactions actually performed against the artifact;
do not invent commands, output, logs, or results.

Sections are numbered after the scenarios of the specification (V4 is scenario M4). Each task
adds the section of the scenario it records.

## Environment and Prerequisites

This section describes the environment of V4 (task T2). Each later section states its own.

- Date and time (UTC): 2026-11-17, 16:53 to 17:09
- Artifact under test: the pointer files of task T2, on top of `develop` at the merge of pull
  request #2905. The probes ran on the files before they were committed (written with `printf`
  and `cp`, see the steps); they were committed unchanged afterwards by
  `docs(agents): [#2904] add a root CLAUDE.md that only imports AGENTS.md` and
  `docs(agents): [#2904] add an import-only CLAUDE.md beside each scoped AGENTS.md`.
- Operating system / environment: Linux 7.0.0-43-generic; Claude Code 2.1.318 (`claude --version`);
  every probe session ran on `claude-sonnet-5-5` (the `init` record of its output).
- Prerequisites and setup performed: two checkouts of the same commit.
  - **Checkout P (a `CLAUDE.md` in a parent folder):** the contributor's working checkout, a
    git worktree inside another checkout of the repository. A `CLAUDE.md` with personal
    instructions exists four folders above it, and the enclosing checkout's `AGENTS.md` three
    folders above it. `ls ~/.claude/CLAUDE.md` reported no such file.
  - **Checkout N (no `CLAUDE.md` above):** made with
    `git clone -q --no-hardlinks --depth 1 "file://<checkout P>" <temporary folder>/m4-clone/torrust-tracker`
    (the `file://` form, so `--depth` applies; `git log --oneline -1` in it printed the merge of
    pull request #2905). The parent folders were checked with
    `p=$(dirname $C); for i in 1 2 3 4 5 6 7 8 9 10; do ls $p/CLAUDE.md $p/AGENTS.md 2>/dev/null; [ "$p" = "/" ] && break; p=$(dirname $p); done`,
    where `$C` is the clone; it printed nothing. The pointer files were copied into it with `cp`
    before the probes that need them.

## Verification Processes

### V1 - The agent files load (scenario M1)

- Goal: show that every agent merged so far is listed, and that no frontmatter key of its file is
  ignored.
- Initial state: the T4 branch with the commit
  `feat(agents): [#2904] add the pr-review-handler agent`; `.claude/agents/` holds only
  `pr-review-handler.md`, with the keys `name`, `description`, `model`, `tools` and `skills`.
  After the run, `docs(agents): [#2904] align pr-review-handler with the review of task T4`
  changed the file's `description` value and its body. Its keys and the values of `name`, `model`,
  `tools` and `skills` did not change, and the `description` was not observed by this run (see
  the results), so the results hold for the file at the pull request's head.
- Environment: 2026-11-19, 08:37:09 to 08:37:20 UTC (`date -u` before and after); Linux 7.0.0-43-generic; Claude Code 2.1.318
  (`claude --version`). One run serves V1 and V2 (session ID `e5b1c5f6-53cf-424c-89b3-4f57d0947d34`).
- Status: `DONE`

#### Steps Performed

One non-interactive session, started at the repository root on a model other than the agent's:

```text
claude -p --model sonnet --debug-file .tmp/m1m2/debug.log --output-format json "This is a verification probe; change nothing, run no command that writes anything. Step 1: list, one per line, the names of every custom agent (subagent type) that you can start and that is defined in this project's .claude/agents/ folder. Step 2: start the pr-review-handler agent with exactly this task: 'This is a verification probe, not a review round. Do not run any command, read any file or change anything. Reply with one line: READY.' Step 3: print the agent's reply verbatim."
```

The debug log stays local, since it also holds the contributor's own settings; the lines quoted
below are copied from it. It was searched with
`grep -n -i -E "frontmatter|ignored|unknown (field|key)"`, and for the agent with
`grep -n -i -E "pr-review-handler|agents/|subagent"`. The run's transcripts, the agent's metadata
and request-cache files, and the JSON result stay local for the same reason. The records V1 and
V2 rely on are stored in [`evidence/m1m2-run-records.md`](evidence/m1m2-run-records.md), each
with the command that printed it.

#### Results

- The session listed `pr-review-handler` as the custom agent of the project, and started it as
  `subagent_type: "pr-review-handler"`; the agent's metadata file records
  `"agentType":"pr-review-handler"`.
- The frontmatter search printed nothing.
- Each key took effect:
  - `name`: the agent type above.
  - `skills`: four debug lines, `[Agent: pr-review-handler] Preloaded skill 'process-pr-review'`,
    and the same for `fetch-review-threads`, `resolve-review-threads` and `commit-changes`; the
    agent's transcript holds the four skills' text right after its task.
  - `model`: see V2.
  - `tools`: the agent's request-cache record lists the tools `Bash`, `Read`, `Edit`, `Write`
    and `SubagentHandback` (the harness's own tool for returning the report). `Grep` and `Glob`,
    which the file also lists, are not there: this build of Claude Code offers no tools of those
    names, and dropped them without a debug line. The file keeps them for the builds that have
    them; `Bash` covers searching in this one.
  - `description`: not observable in this run, which named the agent; it is the text Claude Code
    shows in its agent list and uses to delegate.
- The agent made one tool call, `SubagentHandback` with the message `READY`, and changed nothing.

#### Conclusion

`pr-review-handler` loads, and every key of its file is read: `name`, `model`, `tools` and
`skills` show their effect. Claude Code reports no ignored key; its documentation says an unknown
key is ignored without an error, so the absence of a warning alone would prove little, and the
effects above are the evidence. A listed tool that the build lacks is dropped silently.

### V2 - The pinned model applies (scenario M2)

- Goal: show that `pr-review-handler`, started from a session on another model, runs on the model
  of its frontmatter, `claude-opus-5-5`.
- Initial state, environment and command: the run of V1.
- Status: `DONE`

#### Results

The model is read from the transcript files, not from what the agents say. The table summarizes
the `model` lines stored in [`evidence/m1m2-run-records.md`](evidence/m1m2-run-records.md), with
the `jq` command that printed them:

| Transcript | `model` of every assistant message |
| --- | --- |
| The session (`<session ID>.jsonl`) | `claude-sonnet-5-5` (three entries, two API responses) |
| The agent (`<session ID>/subagents/agent-a6239b9d274951d96.jsonl`) | `claude-opus-5-5` (one entry) |

The agent's request-cache record (`agent-a6239b9d274951d96.prefix.json`) also says
`"model": "claude-opus-5-5"` in its `state` object, and the run's JSON result lists the usage of both models,
`claude-sonnet-5-5` and `claude-opus-5-5`. The agent's metadata file records
`"pickedEffortModel":"claude-sonnet-5-5"`, the session's model, from which the agent's effort
level was taken; it is not the model the agent ran on.

#### Conclusion

The pinned model applied with no fallback. The caller passed no model for the agent, and neither
`CLAUDE_CODE_SUBAGENT_MODEL` nor an organization allow list overrode it in this environment; any
of those would, as the specification's Background states. The first version of this section said
the variable was unset because `printenv` printed nothing, without naming the command or when it
ran. `printenv CLAUDE_CODE_SUBAGENT_MODEL` was run again at 09:14:28 UTC, in the shell of the
session that processed the first review round, and printed nothing (exit 1); the stored records
give the command and its output. What shows the run's model is the agent's transcript,
`claude-opus-5-5`; an override naming the same model would not show in it.

### V3 - A real review round (scenario M3)

- Goal: show that a session that owns a pull request can delegate one real review round to
  `pr-review-handler`, and that the agent takes it from the fetch of the findings to resolved
  threads.
- Initial state: pull request #2963 (this task) open at head
  `docs(issues): [#2904] record the task review of T4 and check AC2`, with the approving review
  `5972182565` of `da2ce7` (six inline findings, F1 to F6) and an overview of Copilot,
  `5972197600`, with no inline comment.
- Environment: 2026-11-19; Linux 7.0.0-43-generic; Claude Code 2.1.318 (`claude --version`). Three
  runs, described below; the session of each ran on `claude-sonnet-5-5`.
- Status: `DONE`

#### Steps Performed

A Claude Code session reads agent files when it starts, so each run is a new non-interactive
session in the branch's checkout. The session was told to start `pr-review-handler` exactly once,
with a fixed task, and to print the agent's report verbatim:

```text
claude -p --model sonnet --debug-file <run's debug log> --output-format json "You are the session that owns pull request #2963 and you delegate one review round. Do not process the review yourself and do not change any file. Start the pr-review-handler agent exactly once, [...] with exactly the task between the markers as its prompt, [...] print its report verbatim and add nothing. ---BEGIN TASK--- <task> ---END TASK---"
```

The task gave the agent what its file names as inputs: the pull request, the poller's event
(`kind` `review`, review ID `5972182565`, its URL, `head_sha` and `cursor`), the commands that act
as the contributor (`gh-bot` for GitHub, `git-bot-push` for the push, a wrapper that runs
`validate-audit-record.py` with the contributor's token), and what the session knew of the round.
It also forbade merging, the ACK, review requests, the approval of workflow runs and history
rewrites. The debug logs, transcripts and JSON results stay local, as in V1. The records V3 relies
on are stored in [`evidence/m3-run-records.md`](evidence/m3-run-records.md), each with the command
that printed it: the `model` of every assistant message, each agent's metadata file and
request-cache record, how each run was started, the lines that show how run 1 ended, and the
reports of the three runs. That file was added in the third review round of pull request #2963
(finding F14); before it, this section gave these records without a stored copy.

| Run | Start (UTC) | How the session started the agent | Agent's transcript: `model` of every assistant message | End |
| --- | --- | --- | --- | --- |
| 1 | 09:12:11 | In the background (metadata `"requestShape":"background"`) | `claude-opus-5-5` (76 entries) | Stopped by the harness at 09:22:27 |
| 2 | 09:23:02 | In the foreground, as the instruction then said | `claude-opus-5-5` (47 entries) | Returned a question at 09:27:39 |
| 3 | 11:19:19 | In the foreground | `claude-opus-5-5` (49 entries) | Returned its report at 11:36:01 |

In every run the agent's request-cache record lists the tools `Bash`, `Read`, `Edit`, `Write` and
`SubagentHandback` and the model `claude-opus-5-5`.

#### Results

- **Run 1.** The first instruction did not say how to start the agent, and the session started it
  in the background. 10 minutes later, while the agent was still working, Claude Code stopped it:
  the debug log says `print wind-down: killing background local_agent task [...] at the wait
  ceiling`, and the session's stderr said `Background tasks still running 10m after the last turn
  (subagent "Process PR 2963 review round"); stopping them. Set
  CLAUDE_CODE_PRINT_BG_WAIT_CEILING_MS=0 to wait indefinitely.` That message of Claude Code is
  the only source for the variable and its value `0`; the specification's table of facts does not
  list it. The agent had made five local, signed commits
  that fix F1 to F5, and had pushed nothing, replied to nothing and created no audit record.
- **Run 2.** The instruction now asked for a foreground start, the variable above was set to `0`,
  and the task told the agent about the five commits. The agent checked them against the tree and
  the review and kept them. It edited the pull request's description for F6. Before its push went
  out it saw that the maintainer had posted an ACK of the reviewed head at 09:24:49 UTC, so it
  stopped and returned the question whether to push the fixes or leave that head for the merge. It
  had pushed nothing and posted no reply. This is the behaviour its file asks for: a decision it
  cannot make goes back as a question.
- **Run 3.** The contributor answered "Push the fixes", and the task carried that answer. The
  agent checked that the pull request was still open, then pushed the five commits. Its first push
  was refused by the pre-push hook: a test of `clippy-allow-reasons` failed while setting up a
  `git` fixture ("File exists"), a failure unrelated to this documentation-only branch; the agent
  ran that package's test target alone (10 of 10 passed, by its report) and pushed again, and `git ls-remote`
  showed the new head. It created the audit record `docs/pr-reviews/pr-2963-review/PR-REVIEW.md`
  (F1 to F6, all `FIXED`), posted one reply in each of the six threads, resolved them, ran
  `validate-audit-record.py` (exit `0`), committed the record as
  `docs(pr-reviews): [#2904] add the audit record of PR #2963 review round 1` and pushed it. Its
  report named the reviews, each finding with its disposition and thread state, the commits, the
  head after the push, the replies, 0 unresolved threads, the validator's exit code and "none" for
  questions.
- **Checked by the session afterwards, not taken from the report:** `git ls-remote` showed the
  audit commit as the branch head; a GraphQL query of the review threads showed the six threads
  resolved, each with a reply of the contributor's account; `validate-audit-record.py` exited `0`
  again.

#### Conclusion

The expected result is met: fixes committed and pushed, every thread replied to and then
resolved, the audit record created, `validate-audit-record.py` exits `0`, and a short report
returned. In all three runs the agent ran on its pinned model.

Two limits for whoever delegates a round:

- A non-interactive session must start the agent in the foreground; otherwise a round longer
  than 10 minutes is cut off. Claude Code's message names
  `CLAUDE_CODE_PRINT_BG_WAIT_CEILING_MS=0` as a way to wait without that limit, but its effect
  alone was not observed: every run that set the variable also asked for a foreground start, and
  its agent ran in the foreground (`"requestShape":"foreground"`). An interactive session is not
  affected.
- A round can take more than one run: the agent stops at a question, and the session starts it
  again with the answer and with what the earlier run left.

### V4 - `AGENTS.md` reaches the agents (scenario M4)

- Goal: show that the session and an agent it starts receive the root `AGENTS.md`, and the rules
  of a scoped `AGENTS.md` after reading a file in its directory, with and without a `CLAUDE.md`
  in a parent folder; and decide whether the scoped pointer files of D11 are needed.
- Initial state: `develop` at the merge of pull request #2905, with no `CLAUDE.md` in the
  repository.
- Status: `DONE`

#### Steps Performed

Each probe is one non-interactive Claude Code session started in the checkout:

```text
claude -p "<prompt>" --tools <tools> --output-format json < /dev/null
```

The session prompt, used with `--tools Read`:

```text
This is a test of which instruction files were loaded into your context. Use the Read tool exactly once, on the file packages/configuration/Cargo.toml. Use no other tool and read no other file. Then answer these three questions from your context only, in three lines:
Q1: Were you given the content of this repository's root AGENTS.md? If yes, quote its first heading line verbatim. If not, answer NOT LOADED.
Q2: Were you given the content of packages/configuration/AGENTS.md? If yes, quote verbatim the heading of its first rule (a line starting with "### Rule:"). If not, answer NOT LOADED.
Q3: List the paths of every CLAUDE.md or AGENTS.md file whose content you were given.
```

The agent prompt, used with `--tools Read Agent`, wraps the session prompt:

```text
This is a test of which instruction files a started agent receives. Start exactly one subagent of type general-purpose with the Agent tool. Give it exactly the task between the markers, and nothing else. Do not read any file yourself. When it answers, print its answer verbatim and add nothing.
---BEGIN TASK---
<the session prompt>
---END TASK---
```

The expected quotes are the first line of `AGENTS.md`
(`# Torrust Tracker — AI Assistant Instructions`) and the first rule heading of
`packages/configuration/AGENTS.md`
(`### Rule: Use typed newtypes for domain-constrained configuration fields`). `--tools Read`
leaves the session no tool but `Read`. The prompt asks for one call on `Cargo.toml`, but nothing
enforces the number of calls: that the session made that one call and no other is recorded only
for probes 9 to 12 (see the tool calls below), which repeat probes 5 to 8.

The probes ran one at a time, in this order. Times are UTC: the modification time of the probe's output file
(`TZ=UTC ls -l --time-style=+%H:%M:%S`) for probes 1 and 2, `date -u` at the end of the probe for probes 3 to 8, and `date -u` at its start
and end for probes 9 to 12.

1. Probe 1, checkout P, no pointer file, session prompt. Output written at 16:54:58.
2. Wrote the root `CLAUDE.md` in checkout P (`printf '@AGENTS.md\n' > CLAUDE.md`).
3. Probe 2, checkout P, root pointer only, session prompt. Output written at 16:55:15.
4. Wrote a `CLAUDE.md` with the same content beside each of the ten scoped `AGENTS.md` files in
   checkout P.
5. Created checkout N from the commit without pointer files.
6. Probe 3, checkout N, no pointer file, session prompt. Ended 17:06:33.
7. Copied the root `CLAUDE.md` into checkout N.
8. Probe 4, checkout N, root pointer only, session prompt. Ended 17:06:52.
9. Copied the ten scoped `CLAUDE.md` files into checkout N; `git status --short | wc -l` printed 11.
10. Probes 5 to 8, all pointer files, with `--output-format json`: checkout N session (ended
    17:07:02), checkout N agent (17:07:19), checkout P session (17:07:30), checkout P agent
    (17:07:45).
11. Probes 9 to 12, the same four cases again with `--output-format stream-json --verbose`, to
    record the tool calls: checkout N session (17:08:05 to 17:08:15), checkout N agent (17:08:15
    to 17:08:30), checkout P session (17:08:30 to 17:08:41), checkout P agent (17:08:41 to
    17:08:57).

#### Observed Result

Answers to Q1 and Q2, and the files each probe listed for Q3. "root pair" is the root `CLAUDE.md`
and `AGENTS.md`; "scoped pairs" is the `CLAUDE.md` and `AGENTS.md` of `packages/` and of
`packages/configuration/`.

| Probe | Checkout | Pointer files | Who answered | Q1 root `AGENTS.md` | Q2 scoped rule | Q3 files given |
| ----- | -------- | ------------- | ------------ | ------------------- | -------------- | -------------- |
| 1 | P | none | session | NOT LOADED | NOT LOADED | only the parent folder's `CLAUDE.md` |
| 2 | P | root only | session | quoted | NOT LOADED | the parent folder's `CLAUDE.md`, root pair |
| 3 | N | none | session | quoted | quoted | root `AGENTS.md`, `packages/AGENTS.md`, `packages/configuration/AGENTS.md` |
| 4 | N | root only | session | quoted | NOT LOADED | root pair |
| 5, 9 | N | all | session | quoted | quoted | root pair, scoped pairs |
| 6, 10 | N | all | started agent | quoted | quoted | root pair, scoped pairs |
| 7, 11 | P | all | session | quoted | quoted | the parent folder's `CLAUDE.md`, root pair, scoped pairs |
| 8, 12 | P | all | started agent | quoted | quoted | the parent folder's `CLAUDE.md`, root pair, scoped pairs |

Every "quoted" answer gave the expected line verbatim. Answers of probes 1, 2 and 4, verbatim:

```text
Probe 1
Q1: NOT LOADED. Only the CLAUDE.md at the Torrust checkouts folder level was given to me, and no root AGENTS.md content appeared in my context.
Q2: NOT LOADED. No `packages/configuration/AGENTS.md` content appeared, even after reading `packages/configuration/Cargo.toml`.

Probe 2
Q1: The root AGENTS.md content was loaded (via the `@AGENTS.md` import in the worktree CLAUDE.md). Its first heading line is: `# Torrust Tracker — AI Assistant Instructions`
Q2: NOT LOADED. Reading `packages/configuration/Cargo.toml` did not bring in a `packages/configuration/AGENTS.md`.

Probe 4
Q1: Yes. The root AGENTS.md was loaded, and its first heading line is `# Torrust Tracker — AI Assistant Instructions`.
Q2: NOT LOADED. I was not given the content of packages/configuration/AGENTS.md.
```

Tool calls recorded by probes 9 to 12 (`tool_use` records of the stream; "main" is the session,
"agent" is a record with a parent tool call):

```text
Probe 9   main:  Read packages/configuration/Cargo.toml
Probe 10  main:  Agent (subagent_type general-purpose)
          agent: Read packages/configuration/Cargo.toml
          agent: SubagentHandback
Probe 11  main:  Read packages/configuration/Cargo.toml
Probe 12  main:  Agent (subagent_type general-purpose)
          agent: Read packages/configuration/Cargo.toml
          agent: SubagentHandback
```

The started agents of probes 6, 8, 10 and 12 said the root pair was in their context at the
start and the scoped pairs arrived after their `Read` call. The answer of probe 12's agent to Q1
and Q2, verbatim:

```text
Q1: Root AGENTS.md was loaded (via CLAUDE.md @AGENTS.md import). First heading line verbatim: "# Torrust Tracker — AI Assistant Instructions"
Q2: packages/configuration/AGENTS.md was loaded. First rule heading verbatim: "### Rule: Use typed newtypes for domain-constrained configuration fields"
```

The answer of every probe is stored in [`evidence/m4-probe-answers.md`](evidence/m4-probe-answers.md),
with four local path fragments replaced. The rest of each output is not stored: it contains the
contributor's personal `CLAUDE.md` from the parent folder.

#### Conclusion

The scenario's expected result is met with all eleven pointer files: in both checkouts the
session and the started agent received the root `AGENTS.md`, and the scoped rule after reading
a file in `packages/configuration/`. Every `CLAUDE.md` in the repository contains only the
import `@AGENTS.md` (`git ls-files | grep -E '(^|/)CLAUDE\.md$' | xargs cat | sort -u` prints
that one line).

The probes also settle the question D11 left to this task:

- Without any `CLAUDE.md` above or in the checkout, Claude Code loads the root and the scoped
  `AGENTS.md` files by itself (probe 3).
- With a `CLAUDE.md` in a parent folder and none in the repository, it loads neither (probe 1).
- Any `CLAUDE.md` at the root or above stops the scoped `AGENTS.md` files from loading (probes
  1, 2 and 4). A root `CLAUDE.md` restores only the root `AGENTS.md`: in checkout P the scoped
  ones stay unloaded (probe 2, as in probe 1), and in checkout N, where they loaded without it
  (probe 3), it stops them (probe 4). So the scoped pointer files are needed, as the reviewer of
  pull request #2905 reported.

Limits of this evidence: the answers are what each model reported about its own context. For
probes 9 to 12 the recorded tool calls show the quoted lines were not read from the files. For
probes 1 to 8 no tool calls were recorded, so a quote there could in principle come from an extra
`Read` call; this matters most for probe 3, the one positive result behind "loads by itself",
which was not repeated; its answer text is in the stored file. The NOT LOADED answers of probes 1, 2 and 4 do not depend on it. Only
`packages/configuration/` was probed, and the other eight scoped directories rely on the same
mechanism; the behaviour is that of Claude Code 2.1.318.

### V6 - Review rounds per pull request (scenario M6)

- Goal: count the review rounds with fixes of the first three pull requests handled by
  `pr-review-handler`, with the model of each round, next to the baseline of pull request #2851:
  14 rounds (1 to 3 Claude Fable 5.2, 4 to 9 Claude Sonnet, 10 to 14 Claude Opus 5.5).
- Method: a round is one processing round that pushed fixes, the first definition of
  `wait-for-pr-review` (Round Limit): "A round is one push of fixes made in response to a review
  event". A round can answer several reviews, when they arrive before its push; a push that
  carries only the audit record adds no round. This is how the baseline was counted: the 14
  rounds of pull request #2851 are the 14 rounds of its audit record
  (`docs/pr-reviews/pr-2851-review/PR-REVIEW.md`, section Findings), each ending in one commit
  "record round N in the review audit of PR #2851", and rounds 1, 5, 6, 8 and 10 answer two
  reviews each. The same Round Limit section also counts "the number of distinct reviews in the
  audit record that have at least one `FIXED` finding"; the two disagree when one round answers
  two reviews, so that count is given too: 19 for pull request #2851 (all 19 of its reviews
  have a `FIXED` finding), and the reviews listed below for each pull request. The model of a
  round is the one in its agent's transcript. The count of a pull request is final when it
  merges. The `model` lines of each round's agent transcripts are stored in
  [`evidence/m3-run-records.md`](evidence/m3-run-records.md), with the command that printed them.
- Status: `IN_PROGRESS` (one of three pull requests)

| Pull request | Rounds with fixes | Model of each round | Count final |
| --- | --- | --- | --- |
| #2963 (T4) | 4 so far, answering 5 reviews with a `FIXED` finding. Round 1: review `5972182565`, F1 to F6, all fixed (see V3). Round 2: review `5974337471`, F8 to F13, five fixed and F8 with no change. Round 3: reviews `5974503813` and `5974634671`, F14 to F18, all fixed. Round 4: review `5976833863`, F20, fixed | Round 1: `claude-opus-5-5` (three runs). Round 2: `claude-opus-5-5`. Round 3: `claude-opus-5-5` (two runs). Round 4: `claude-opus-5-5` (two runs) | No, the pull request is open |

## Failures and Follow-up

- A first attempt at steps 5 to 10 did not run: `git clone --local` failed with "Invalid
  cross-device link", and a shell loop in the same command then ran without end and was stopped.
  No probe ran in that attempt. The clone was repeated with `--no-hardlinks`.
- Correction to the description of checkout P. The first version of this document, in
  `docs(issues): [#2904] record scenario M4 and mark task T2 done`, said the personal `CLAUDE.md`
  was three folders above checkout P and did not say that checkout P is a git worktree inside
  another checkout of the repository. It is four folders above, and the enclosing checkout's
  `AGENTS.md` is three folders above. The Environment section was corrected in
  `docs(issues): [#2904] correct the M4 evidence and store every probe's answer`. No probe was
  rerun and the conclusions do not depend on it: the enclosing `AGENTS.md` was not loaded in
  probe 1, and no probe listed it.
- In probe 12 the session printed "The subagent is still running" before it printed the agent's
  answer; the answer recorded above is the one it printed afterwards.
