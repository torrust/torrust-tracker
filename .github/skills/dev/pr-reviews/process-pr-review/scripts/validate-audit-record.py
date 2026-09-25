#!/usr/bin/env python3
"""Validate a PR review audit record against GitHub data and branch history.

Usage: validate-audit-record.py --pr-number <number> [options]

Checks that docs/pr-reviews/pr-<PR_NUMBER>-review/PR-REVIEW.md is internally
consistent and matches the pull request it describes. Finding IDs may be
audit-local (F1) or reviewer-provided (OPS-001):

  - every tracking row has a detail entry with the same finding ID, and vice versa
  - every discussion-anchored row's Source review ID equals the source comment's
    pull_request_review_id
  - every review-anchored row's Source review ID equals the review in its Source URL
  - every discussion-anchored row cites exactly one Reply URL, and that reply is
    posted on the row's own source thread (in_reply_to_id == source comment id)
  - every Resolution reference commit subject exists in <base>..HEAD
  - every row's Severity equals the [Severity] bracket of its source comment
    (qualifiers after the severity word, such as "(inferred)", are ignored)
  - Processing Log entries are in chronological order

Options:
  --pr-number <number>     Pull request number (required)
  --audit-file <path>      Audit record (default: docs/pr-reviews/pr-<N>-review/PR-REVIEW.md)
  --comments-file <path>   REST review-comments JSON; fetched with gh when omitted
  --owner <owner>          Repository owner (default: torrust)
  --repo <repo>            Repository name (default: torrust-tracker)
  --base <ref>             Base ref for commit-subject lookup (default: develop)
  -h, --help               Show this help

Output:
  - One JSON summary line to stdout:
      {"status":"ok"|"failed","rows":N,"log_entries":N,"failures":N}
  - One failure per line to stderr
  - Exit 0 when consistent, 1 on any failure, 2 on usage error
"""

import argparse
import json
import re
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path

# Audit-local IDs (F1) and reviewer-provided IDs (OPS-001, PERSISTENT-RUNNER-PRIVILEGE).
FINDING_ID = r"[A-Z][A-Z0-9]*(?:-[A-Z0-9]+)*"

ROW_RE = re.compile(rf"^\| ({FINDING_ID}) \| [^|]+ \| [^|]+ \| (\w+)[^|]* \|", re.M)
DETAIL_SPLIT_RE = re.compile(rf"^### ({FINDING_ID}) - .*$", re.M)
DISCUSSION_RE = re.compile(r"discussion_r(\d+)")
REVIEW_RE = re.compile(r"pullrequestreview-(\d+)")
LOG_ENTRY_RE = re.compile(r"^- (\d{4}-\d{2}-\d{2} \d{2}:\d{2}) UTC - ", re.M)
SEVERITY_BRACKET_RE = re.compile(r"^\[(\w+)\]")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(add_help=False)
    parser.add_argument("--pr-number", type=int)
    parser.add_argument("--audit-file")
    parser.add_argument("--comments-file")
    parser.add_argument("--owner", default="torrust")
    parser.add_argument("--repo", default="torrust-tracker")
    parser.add_argument("--base", default="develop")
    parser.add_argument("-h", "--help", action="store_true")
    args = parser.parse_args()
    if args.help:
        print(__doc__)
        sys.exit(0)
    if args.pr_number is None:
        print("Error: --pr-number is required.", file=sys.stderr)
        print(__doc__, file=sys.stderr)
        sys.exit(2)
    if args.audit_file is None:
        args.audit_file = f"docs/pr-reviews/pr-{args.pr_number}-review/PR-REVIEW.md"
    return args


def load_comments(args: argparse.Namespace) -> dict[int, dict]:
    if args.comments_file:
        raw = Path(args.comments_file).read_text()
    else:
        print(f"Fetching review comments for {args.owner}/{args.repo} PR #{args.pr_number}...", file=sys.stderr)
        raw = subprocess.run(
            ["gh", "api", f"repos/{args.owner}/{args.repo}/pulls/{args.pr_number}/comments", "--paginate"],
            capture_output=True,
            text=True,
            check=True,
        ).stdout
    # --paginate can emit concatenated arrays; json.JSONDecoder.raw_decode handles that.
    decoder = json.JSONDecoder()
    comments: list[dict] = []
    pos = 0
    raw = raw.strip()
    while pos < len(raw):
        chunk, pos = decoder.raw_decode(raw, pos)
        comments.extend(chunk)
        while pos < len(raw) and raw[pos].isspace():
            pos += 1
    return {c["id"]: c for c in comments}


def branch_subjects(base: str) -> set[str]:
    out = subprocess.run(
        ["git", "log", "--format=%s", f"{base}..HEAD"], capture_output=True, text=True, check=True
    ).stdout
    return set(out.splitlines())


def field(body: str, name: str) -> str | None:
    match = re.search(rf"^- {re.escape(name)}: (.*)$", body, re.M)
    return match.group(1).strip() if match else None


def check_row(fid: str, severity: str, body: str, comments: dict[int, dict], subjects: set[str]) -> list[str]:
    failures: list[str] = []
    source_url = field(body, "Source URL") or ""
    review_id = field(body, "Source review ID") or ""
    reply_field = field(body, "Reply URL") or ""

    discussion = DISCUSSION_RE.search(source_url)
    review = REVIEW_RE.search(source_url)

    if discussion:
        cid = int(discussion.group(1))
        comment = comments.get(cid)
        if comment is None:
            failures.append(f"{fid}: source comment r{cid} not found in review comments")
        else:
            if str(comment["pull_request_review_id"]) != review_id:
                failures.append(
                    f"{fid}: Source review ID {review_id} != r{cid}'s review {comment['pull_request_review_id']}"
                )
            bracket = SEVERITY_BRACKET_RE.match(comment["body"].split("\n", 1)[0])
            if bracket and bracket.group(1) != severity:
                failures.append(f"{fid}: severity {severity} != source bracket {bracket.group(1)}")
        reply_ids = DISCUSSION_RE.findall(reply_field)
        if len(reply_ids) != 1:
            failures.append(f"{fid}: expected exactly one discussion Reply URL, found {len(reply_ids)}")
        for rid in reply_ids:
            reply = comments.get(int(rid))
            if reply is None:
                failures.append(f"{fid}: reply r{rid} does not exist")
            elif reply.get("in_reply_to_id") != cid:
                failures.append(f"{fid}: reply r{rid} is on thread {reply.get('in_reply_to_id')}, not {cid}")
    elif review:
        if review.group(1) != review_id:
            failures.append(f"{fid}: Source review ID {review_id} != Source URL review {review.group(1)}")
    else:
        failures.append(f"{fid}: Source URL is neither a discussion nor a review URL")

    resolution = field(body, "Resolution reference") or ""
    for ref in resolution.split(";"):
        ref = ref.strip().strip("`<>")
        if not ref or ref.startswith("http"):
            continue
        if ref not in subjects:
            failures.append(f"{fid}: resolution reference not a commit subject on branch: {ref}")
    return failures


def main() -> int:
    args = parse_args()
    audit_path = Path(args.audit_file)
    if not audit_path.is_file():
        print(f"Error: audit file '{audit_path}' does not exist.", file=sys.stderr)
        return 2
    text = audit_path.read_text()
    comments = load_comments(args)
    subjects = branch_subjects(args.base)

    failures: list[str] = []

    rows = {fid: sev for fid, sev in ROW_RE.findall(text)}
    parts = DETAIL_SPLIT_RE.split(text)[1:]
    details = {parts[i]: parts[i + 1] for i in range(0, len(parts), 2)}
    if set(rows) != set(details):
        failures.append(f"tracking rows and detail entries differ: {sorted(set(rows) ^ set(details))}")

    for fid in (f for f in rows if f in details):
        failures.extend(check_row(fid, rows[fid], details[fid], comments, subjects))

    log_section = text.split("## Processing Log", 1)
    log_stamps: list[datetime] = []
    if len(log_section) == 2:
        log_stamps = [
            datetime.strptime(s, "%Y-%m-%d %H:%M").replace(tzinfo=timezone.utc)
            for s in LOG_ENTRY_RE.findall(log_section[1])
        ]
        if log_stamps != sorted(log_stamps):
            failures.append("Processing Log entries are not in chronological order")

    for failure in failures:
        print(failure, file=sys.stderr)
    status = "ok" if not failures else "failed"
    print(json.dumps({"status": status, "rows": len(rows), "log_entries": len(log_stamps), "failures": len(failures)}))
    return 0 if not failures else 1


if __name__ == "__main__":
    sys.exit(main())
