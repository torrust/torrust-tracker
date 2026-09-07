#!/usr/bin/env python3
"""Reproducibly verify the release-binary scenarios for issue #2151.

Build the binary first with:
    cargo build --release --bin torrust-tracker

The script writes temporary configurations and evidence only to the repository's
ignored `.tmp/issue-2151-manual/` directory.
"""

from __future__ import annotations

import os
import re
import shutil
import signal
import socket
import subprocess
import time
import urllib.request
from pathlib import Path

ROOT = Path(__file__).resolve().parents[4]
WORK = ROOT / ".tmp" / "issue-2151-manual"
BINARY = ROOT / "target" / "release" / "torrust-tracker"
SOURCE_ENV = ("TORRUST_TRACKER_CONFIG_TOML", "TORRUST_TRACKER_CONFIG_TOML_PATH")
SUMMARY = WORK / "summary.txt"


def record(message: str) -> None:
    with SUMMARY.open("a", encoding="utf-8") as summary:
        summary.write(f"{message}\n")


def config(health_port: int, storage_path: Path | None = None) -> str:
    database = ""
    if storage_path:
        storage_path.mkdir(parents=True, exist_ok=True)
        database = f'''\n[core.database]\ndriver = "sqlite3"\npath = "{storage_path / "sqlite3.db"}"\n'''

    return f'''[metadata]
app = "torrust-tracker"
purpose = "configuration"
schema_version = "3.0.0"

[logging]
trace_filter = "info"

[core]
listed = false
private = false
{database}
[health_check_api]
bind_address = "127.0.0.1:{health_port}"
'''


def clean_environment(extra: dict[str, str] | None = None) -> dict[str, str]:
    environment = os.environ.copy()
    for name in SOURCE_ENV:
        environment.pop(name, None)
    if extra:
        environment.update(extra)
    return environment


def start(name: str, arguments: list[str], extra: dict[str, str] | None = None) -> subprocess.Popen[str]:
    log = (WORK / f"{name}.log").open("w", encoding="utf-8")
    process = subprocess.Popen(
        [str(BINARY), *arguments],
        cwd=ROOT,
        env=clean_environment(extra),
        stdout=log,
        stderr=subprocess.STDOUT,
        text=True,
    )
    process._issue_2151_log = log  # type: ignore[attr-defined]
    return process


def finish(process: subprocess.Popen[str], timeout: float = 30) -> tuple[int, str]:
    if process.poll() is None:
        process.send_signal(signal.SIGTERM)
    try:
        code = process.wait(timeout)
    except subprocess.TimeoutExpired:
        process.kill()
        code = process.wait(5)
        raise AssertionError(f"process timed out and was killed with {code}")
    process._issue_2151_log.close()  # type: ignore[attr-defined]
    return code, Path(process._issue_2151_log.name).read_text(encoding="utf-8")  # type: ignore[attr-defined]


def wait_for_health(port: int, process: subprocess.Popen[str]) -> None:
    deadline = time.monotonic() + 10
    url = f"http://127.0.0.1:{port}/health_check"
    while time.monotonic() < deadline:
        if process.poll() is not None:
            _, output = finish(process)
            raise AssertionError(f"process exited before health endpoint was ready:\n{output}")
        try:
            with urllib.request.urlopen(url, timeout=0.5) as response:
                if response.status == 200:
                    return
        except OSError:
            pass
        time.sleep(0.05)
    _, output = finish(process)
    raise AssertionError(f"timed out waiting for {url}:\n{output}")


def wait_for_port_zero_health(process: subprocess.Popen[str]) -> int:
    deadline = time.monotonic() + 10
    pattern = re.compile(r"HEALTH CHECK API.*Started on: http://127\.0\.0\.1:(\d+)")
    while time.monotonic() < deadline:
        if process.poll() is not None:
            _, output = finish(process)
            raise AssertionError(f"process exited before readiness:\n{output}")
        log_path = Path(process._issue_2151_log.name)  # type: ignore[attr-defined]
        match = pattern.search(log_path.read_text(encoding="utf-8"))
        if match:
            port = int(match.group(1))
            wait_for_health(port, process)
            return port
        time.sleep(0.05)
    _, output = finish(process)
    raise AssertionError(f"timed out discovering port-zero health endpoint:\n{output}")


def assert_exits(name: str, arguments: list[str], expected_code: int, expected_text: str, cwd: Path = ROOT) -> None:
    result = subprocess.run(
        [str(BINARY), *arguments],
        cwd=cwd,
        env=clean_environment(),
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        timeout=10,
    )
    (WORK / f"{name}.log").write_text(result.stdout, encoding="utf-8")
    assert result.returncode == expected_code, (name, result.returncode, result.stdout)
    assert expected_text in result.stdout, (name, expected_text, result.stdout)
    record(
        f"{name}: command={BINARY} {' '.join(arguments)!r}; cwd={cwd}; "
        f"exit={result.returncode}; expected-text={expected_text!r}; PASS"
    )


def assert_port_is_available(port: int) -> None:
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as listener:
        listener.bind(("127.0.0.1", port))
    record(f"listener-probe: bind 127.0.0.1:{port} after failed startup; PASS (port available)")


def main() -> None:
    if not BINARY.is_file():
        raise SystemExit(f"missing release binary; run `cargo build --release --bin torrust-tracker`: {BINARY}")
    shutil.rmtree(WORK, ignore_errors=True)
    WORK.mkdir(parents=True)

    # M1: CLI-only source.
    m1 = WORK / "m1.toml"
    m1.write_text(config(43151), encoding="utf-8")
    process = start("m1", ["--config-toml-path", str(m1)])
    wait_for_health(43151, process)
    code, output = finish(process)
    assert code == 0 and "successfully shutdown" in output, output

    # M2: CLI path outranks both environment base sources.
    cli = WORK / "m2-cli.toml"
    env_path = WORK / "m2-env-path.toml"
    cli.write_text(config(43152), encoding="utf-8")
    env_path.write_text(config(43153), encoding="utf-8")
    process = start(
        "m2",
        ["--config-toml-path", str(cli)],
        {"TORRUST_TRACKER_CONFIG_TOML": config(43154), "TORRUST_TRACKER_CONFIG_TOML_PATH": str(env_path)},
    )
    wait_for_health(43152, process)
    code, output = finish(process)
    assert code == 0, output

    # M3: override applies over a CLI-selected base file.
    m3 = WORK / "m3.toml"
    m3.write_text(config(43155), encoding="utf-8")
    process = start(
        "m3",
        ["--config-toml-path", str(m3)],
        {"TORRUST_TRACKER_CONFIG_OVERRIDE_HEALTH_CHECK_API__BIND_ADDRESS": "127.0.0.1:43156"},
    )
    wait_for_health(43156, process)
    code, output = finish(process)
    assert code == 0, output

    # M4: usage and strict source errors; no source is valid or reaches a listener.
    assert_exits("m4-missing-value", ["--config-toml-path"], 2, "a value is required")
    assert_exits("m4-empty-value", ["--config-toml-path", ""], 2, "must not be empty")
    missing = WORK / "does-not-exist.toml"
    assert_exits("m4-missing-file", ["--config-toml-path", str(missing)], 1, str(missing))
    assert_exits("m4-directory", ["--config-toml-path", str(WORK)], 1, str(WORK))
    malformed = WORK / "malformed.toml"
    malformed.write_text(f"{config(43158)}" "malformed_key = [", encoding="utf-8")
    assert_exits("m4-malformed", ["--config-toml-path", str(malformed)], 1, str(malformed))
    assert_port_is_available(43158)
    unreadable = WORK / "unreadable.toml"
    unreadable.write_text(config(43159), encoding="utf-8")
    unreadable.chmod(0)
    try:
        record(f"m4-unreadable: path={unreadable}; regular-file={unreadable.is_file()}; mode={unreadable.stat().st_mode & 0o777:o}")
        assert_exits("m4-unreadable", ["--config-toml-path", str(unreadable)], 1, str(unreadable))
    finally:
        unreadable.chmod(0o600)
    parent = WORK / "parent"
    child = parent / "child"
    child.mkdir(parents=True)
    (parent / "tracker.toml").write_text(config(43157), encoding="utf-8")
    assert_exits("m4-parent-only-relative", ["--config-toml-path", "tracker.toml"], 1, "tracker.toml", child)
    assert_port_is_available(43157)

    # M5: two port-zero CLI-selected children, independent paths and endpoints.
    first = WORK / "m5-first.toml"
    second = WORK / "m5-second.toml"
    first.write_text(config(0, WORK / "m5-first-storage"), encoding="utf-8")
    second.write_text(config(0, WORK / "m5-second-storage"), encoding="utf-8")
    first_process = start("m5-first", ["-c", str(first)])
    second_process = start("m5-second", ["-c", str(second)])
    first_port = wait_for_port_zero_health(first_process)
    second_port = wait_for_port_zero_health(second_process)
    assert first_port != second_port, (first_port, second_port)
    first_code, first_output = finish(first_process)
    second_code, second_output = finish(second_process)
    assert first_code == second_code == 0
    assert "successfully shutdown" in first_output and "successfully shutdown" in second_output

    print(f"Manual verification passed; evidence: {WORK}")


if __name__ == "__main__":
    main()
