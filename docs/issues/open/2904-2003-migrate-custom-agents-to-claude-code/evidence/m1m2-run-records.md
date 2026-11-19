# Scenarios M1 and M2: the records of the run

<!-- cspell:ignore Byxp Byxgw toolu Usjz -->

The records that sections V1 and V2 of `../manual-verification-evidence.md` rely on, copied from
the files of the probe run with session ID `e5b1c5f6-53cf-424c-89b3-4f57d0947d34` (2026-11-19,
08:37:09 to 08:37:20 UTC). The files themselves stay local: the transcripts and the debug log also
hold the contributor's own instructions and settings. The records below are the output of the
commands named in each section, run on 2026-11-19 at 09:14:28 UTC (`date -u` before them) at the
root of the T4 checkout, by the agent session that processed the first review round of pull
request #2963, on the contributor's machine. The last section gives its own time.

Path names used in the commands, replaced so that the contributor's local folder layout is not
published:

- `$S` is the session's transcript, `<projects-folder>/<session ID>.jsonl`, where
  `<projects-folder>` is the folder in which Claude Code keeps the transcripts of this checkout.
- `$A` is `<projects-folder>/<session ID>/subagents/agent-a6239b9d274951d96`, the common prefix of
  the agent's files: the transcript `$A.jsonl`, the metadata file `$A.meta.json` and the
  request-cache record `$A.prefix.json`.
- `.tmp/m1m2/` is the run's own folder in the checkout (git-ignored), with the debug log
  `debug.log`, the JSON result `result.json` and the start time `start.txt`.

## The `model` of every assistant message

```text
jq -c 'select(.type=="assistant") | {type, timestamp, requestId, model: .message.model}' "$S"
```

```text
{"type":"assistant","timestamp":"2026-11-19T08:37:13.447Z","requestId":"req_011ChByxSY4WV38rK7DFwWg2","model":"claude-sonnet-5-5"}
{"type":"assistant","timestamp":"2026-11-19T08:37:14.092Z","requestId":"req_011ChByxSY4WV38rK7DFwWg2","model":"claude-sonnet-5-5"}
{"type":"assistant","timestamp":"2026-11-19T08:37:17.699Z","requestId":"req_011ChByxpVAY5C1ZoWz2XVto","model":"claude-sonnet-5-5"}
```

The same command on `$A.jsonl`:

```text
{"type":"assistant","timestamp":"2026-11-19T08:37:15.646Z","requestId":"req_011ChByxgwEt87v9p4JxPyTg","model":"claude-opus-5-5"}
```

## The agent's metadata file

```text
jq -c . "$A.meta.json"
```

```text
{"agentType":"pr-review-handler","isBuiltIn":false,"description":"Verification probe","toolUseId":"toolu_0196FoCru3k7Usjz5zDJMugZ","spawnDepth":1,"requestShape":"foreground","requestNonInteractive":true,"pickedEffortModel":"claude-sonnet-5-5","finished":true}
```

## The agent's request-cache record

The fields that V1 and V2 cite, selected from the record's `state` object (it also holds hashes,
beta flags and sizes, which are not copied):

```text
jq -c '.state | {toolNames, model, buildVersion}' "$A.prefix.json"
```

```text
{"toolNames":["Bash","Read","Edit","Write","SubagentHandback"],"model":"claude-opus-5-5","buildVersion":"2.1.318"}
```

## The run's JSON result

```text
jq -c '{session_id, modelUsage: (.modelUsage | keys)}' .tmp/m1m2/result.json
cat .tmp/m1m2/start.txt
```

```text
{"session_id":"e5b1c5f6-53cf-424c-89b3-4f57d0947d34","modelUsage":["claude-opus-5-5","claude-sonnet-5-5"]}
2026-11-19T08:37:09Z
```

## The debug log

```text
grep -n "Preloaded skill" .tmp/m1m2/debug.log
grep -c -i -E "frontmatter|ignored|unknown (field|key)" .tmp/m1m2/debug.log
```

```text
324:2026-11-19T08:37:14.473Z [DEBUG] [Agent: pr-review-handler] Preloaded skill 'process-pr-review'
325:2026-11-19T08:37:14.473Z [DEBUG] [Agent: pr-review-handler] Preloaded skill 'fetch-review-threads'
326:2026-11-19T08:37:14.473Z [DEBUG] [Agent: pr-review-handler] Preloaded skill 'resolve-review-threads'
327:2026-11-19T08:37:14.473Z [DEBUG] [Agent: pr-review-handler] Preloaded skill 'commit-changes'
0
```

## `CLAUDE_CODE_SUBAGENT_MODEL`

The record of V2 before this file did not name the command or its time. The check was repeated
with the commands above, in the same shell, at 2026-11-19 09:14:28 UTC:

```text
printenv CLAUDE_CODE_SUBAGENT_MODEL; echo "exit=$?"
```

```text
exit=1
```

`printenv` printed nothing and exited 1: the variable was not set in that shell. That shell is
not the run's own environment, and it ran 37 minutes after the run.

## The session's listing and the agent's reply

Added in the second review round of pull request #2963 (finding F10). The commands ran on
2026-11-19 at 11:51:19 UTC (`date -u` before them), at the root of the same T4 checkout, by the
`pr-review-handler` agent that processed that round. The session's listing of step 1 is in its transcript; the run's JSON
result holds only the text of step 3.

```text
jq -c 'select(.type=="assistant") | .message.content[] | select(.type=="text" or .type=="tool_use") | if .type=="text" then {text} else {tool_use: .name, input} end' "$S"
```

```text
{"text":"Step 1: the custom agent defined in this project's `.claude/agents/` folder is:\n\n- pr-review-handler\n\nI didn't read the folder. I'm going by the agent list I was given, and that list doesn't say which folder each agent comes from. The other agents on it are built in or come from elsewhere.\n\nStep 2: starting the agent now."}
{"tool_use":"Agent","input":{"description":"Verification probe","subagent_type":"pr-review-handler","prompt":"This is a verification probe, not a review round. Do not run any command, read any file or change anything. Reply with one line: READY.","run_in_background":false}}
{"text":"Step 3: the agent's reply, verbatim:\n\nREADY\n\nThe probe didn't change anything. The agent made one tool call, which it was told not to do, but it reported no change and I haven't checked what the call was."}
```

```text
jq -r .result .tmp/m1m2/result.json
```

```text
Step 3: the agent's reply, verbatim:

READY

The probe didn't change anything. The agent made one tool call, which it was told not to do, but it reported no change and I haven't checked what the call was.
```
