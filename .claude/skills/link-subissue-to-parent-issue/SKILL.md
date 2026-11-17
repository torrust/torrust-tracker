---
name: link-subissue-to-parent-issue
description: Link an existing GitHub issue as a sub-issue of a parent issue or EPIC through the REST API. Use when setting a parent issue or adding a sub-issue to an EPIC.
metadata:
  author: torrust
  version: "1.0"
---

# Linking a Sub-Issue to a Parent Issue

This skill covers the workflow for linking an existing GitHub issue under a parent issue.

## When to Use

Use this when:

- A child issue already exists and needs to be attached to an epic or parent issue
- You need to set or fix the parent issue of an existing sub-issue
- You want to verify that a sub-issue link was created correctly

## Important Detail

The GitHub sub-issues REST API expects the **internal GitHub issue ID** for the child issue,
not the visible issue number.

- Issue number example: `1715`
- Internal issue ID example: `4349463336`

If you send the issue number as `sub_issue_id`, GitHub returns a `422` validation error.

## Standard Workflow

### 1. Confirm the parent and child issue numbers

Decide which issue is the parent and which is the child.

- Parent issue number: the epic or container issue
- Child issue number: the issue to attach under the parent

### 2. Get the internal ID for the child issue

```bash
gh api /repos/torrust/torrust-tracker/issues/{child-issue-number} --jq '.id'
```

Example:

```bash
gh api /repos/torrust/torrust-tracker/issues/1715 --jq '.id'
```

### 3. Link the child issue to the parent issue

```bash
gh api \
  --method POST \
  -H "Accept: application/vnd.github+json" \
  -H "X-GitHub-Api-Version: 2022-11-28" \
  /repos/torrust/torrust-tracker/issues/{parent-issue-number}/sub_issues \
  --input - <<'EOF'
{"sub_issue_id": {child-internal-id}}
EOF
```

Example:

```bash
gh api \
  --method POST \
  -H "Accept: application/vnd.github+json" \
  -H "X-GitHub-Api-Version: 2022-11-28" \
  /repos/torrust/torrust-tracker/issues/1525/sub_issues \
  --input - <<'EOF'
{"sub_issue_id": 4349463336}
EOF
```

### 4. Verify the link

Check the child issue's `parent_issue_url`:

```bash
gh api /repos/torrust/torrust-tracker/issues/{child-issue-number} --jq '.parent_issue_url'
```

Example:

```bash
gh api /repos/torrust/torrust-tracker/issues/1715 --jq '.parent_issue_url'
```

Expected result:

```text
https://api.github.com/repos/torrust/torrust-tracker/issues/1525
```

## Common Failure Modes

### `422` Invalid property `/sub_issue_id`

Cause: you passed the child issue number instead of the child's internal issue ID.

Fix: fetch the child issue with `gh api ... --jq '.id'` and use that value.

### `404 Not Found`

Possible causes:

- Wrong repository path
- Wrong parent issue number
- Missing permissions for sub-issue management
- The repository or issue does not support the operation in the current context

Fix: verify the repo, the parent issue number, and your GitHub permissions.

## Optional MCP Alternative

If GitHub MCP tools are available, prefer the dedicated sub-issue tool over raw API calls.
Still make sure you pass the **internal issue ID** for the child issue, not the issue number.

## Notes for Torrust Tracker

- Parent issues are often EPICs in `docs/issues/`
- Child issues usually have their own spec file and implementation branch
- After creating and linking a new issue, rename the local spec file to include the assigned issue number
