#!/usr/bin/env python3

import argparse
import atexit
import concurrent.futures
import hashlib
import http.client
import json
import math
import os
import shutil
import signal
import socket
import statistics
import subprocess
import tempfile
import threading
import time
import urllib.parse
import urllib.request
from pathlib import Path


PLAYGROUND_DIR = Path("/Users/crab/Documents/Playground")
DEFAULT_BEFORE_REPO = PLAYGROUND_DIR / "torrust-tracker-before-bench"
DEFAULT_AFTER_REPO = PLAYGROUND_DIR / "torrust-tracker-work"
API_TOKEN = "CodexBenchmarkToken"


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Benchmark before/after tracker persistence behavior across sqlite3, mysql, and postgresql."
    )
    parser.add_argument("--before-repo", type=Path, default=DEFAULT_BEFORE_REPO)
    parser.add_argument("--after-repo", type=Path, default=DEFAULT_AFTER_REPO)
    parser.add_argument("--dbs", nargs="+", choices=("sqlite3", "mysql", "postgresql"), default=["sqlite3", "mysql", "postgresql"])
    parser.add_argument("--mysql-version", default="8.4")
    parser.add_argument("--postgres-version", default="16")
    parser.add_argument("--ops", type=int, default=200)
    parser.add_argument("--reload-iterations", type=int, default=30)
    parser.add_argument("--concurrency", type=int, default=16)
    parser.add_argument("--skip-build", action="store_true")
    parser.add_argument("--json-output", type=Path)
    return parser.parse_args()


def run_command(*args: str, cwd: Path | None = None, env: dict[str, str] | None = None, check: bool = True) -> subprocess.CompletedProcess:
    return subprocess.run(
        args,
        cwd=cwd,
        env=env,
        text=True,
        capture_output=True,
        check=check,
    )


def choose_free_port() -> int:
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as sock:
        sock.bind(("127.0.0.1", 0))
        return sock.getsockname()[1]


def wait_for_http_ok(url: str, timeout_seconds: int) -> None:
    deadline = time.time() + timeout_seconds
    last_error = None
    while time.time() < deadline:
        try:
            with urllib.request.urlopen(url, timeout=2) as response:
                if response.status == 200:
                    return
        except Exception as err:  # noqa: BLE001
            last_error = err
        time.sleep(0.25)
    raise RuntimeError(f"Timed out waiting for {url!r}: {last_error}")


def docker_exec(container: str, *args: str, check: bool = True) -> subprocess.CompletedProcess:
    return run_command("docker", "exec", container, *args, check=check)


def wait_for_mysql(container: str, timeout_seconds: int) -> None:
    deadline = time.time() + timeout_seconds
    while time.time() < deadline:
        result = docker_exec(
            container,
            "sh",
            "-lc",
            "mysqladmin ping -h 127.0.0.1 --password=test --silent",
            check=False,
        )
        if result.returncode == 0:
            return
        time.sleep(1)
    raise RuntimeError(f"Timed out waiting for MySQL container {container!r}")


def wait_for_postgres(container: str, timeout_seconds: int) -> None:
    deadline = time.time() + timeout_seconds
    while time.time() < deadline:
        result = docker_exec(
            container,
            "sh",
            "-lc",
            "pg_isready -U postgres -d torrust_tracker",
            check=False,
        )
        if result.returncode == 0:
            return
        time.sleep(1)
    raise RuntimeError(f"Timed out waiting for PostgreSQL container {container!r}")


def build_tracker_binary(repo_path: Path) -> None:
    run_command("cargo", "build", "--release", "--bin", "torrust-tracker", cwd=repo_path)


def start_database(
    driver: str,
    workspace: Path,
    mysql_version: str,
    postgres_version: str,
    cleanup_items: list[tuple[str, str]],
) -> str:
    if driver == "sqlite3":
        return str(workspace / "tracker.sqlite3.db")

    if driver == "mysql":
        host_port = choose_free_port()
        container = f"torrust-bench-mysql-{os.getpid()}-{host_port}"
        run_command(
            "docker",
            "run",
            "-d",
            "--rm",
            "--name",
            container,
            "-e",
            "MYSQL_ROOT_HOST=%",
            "-e",
            "MYSQL_ROOT_PASSWORD=test",
            "-e",
            "MYSQL_DATABASE=torrust_tracker",
            "-p",
            f"127.0.0.1:{host_port}:3306",
            f"mysql:{mysql_version}",
            "--default-authentication-plugin=mysql_native_password",
        )
        cleanup_items.append(("container", container))
        wait_for_mysql(container, 180)
        return f"mysql://root:test@127.0.0.1:{host_port}/torrust_tracker"

    host_port = choose_free_port()
    container = f"torrust-bench-postgres-{os.getpid()}-{host_port}"
    run_command(
        "docker",
        "run",
        "-d",
        "--rm",
        "--name",
        container,
        "-e",
        "POSTGRES_PASSWORD=test",
        "-e",
        "POSTGRES_USER=postgres",
        "-e",
        "POSTGRES_DB=torrust_tracker",
        "-p",
        f"127.0.0.1:{host_port}:5432",
        f"postgres:{postgres_version}",
    )
    cleanup_items.append(("container", container))
    wait_for_postgres(container, 180)
    return f"postgresql://postgres:test@127.0.0.1:{host_port}/torrust_tracker"


def write_tracker_config(workspace: Path, driver: str, database_path: str, tracker_port: int, api_port: int, health_port: int) -> Path:
    config_path = workspace / "tracker.toml"
    config_path.write_text(
        "\n".join(
            [
                '[metadata]',
                'app = "torrust-tracker"',
                'purpose = "configuration"',
                'schema_version = "2.0.0"',
                "",
                "[logging]",
                'threshold = "error"',
                "",
                "[core]",
                "listed = false",
                "private = false",
                "",
                "[core.database]",
                f'driver = "{driver}"',
                f'path = "{database_path}"',
                "",
                "[[http_trackers]]",
                f'bind_address = "127.0.0.1:{tracker_port}"',
                "",
                "[http_api]",
                f'bind_address = "127.0.0.1:{api_port}"',
                "",
                "[http_api.access_tokens]",
                f'admin = "{API_TOKEN}"',
                "",
                "[health_check_api]",
                f'bind_address = "127.0.0.1:{health_port}"',
                "",
            ]
        ),
        encoding="utf-8",
    )
    return config_path


def start_tracker(repo_path: Path, config_path: Path, log_path: Path, cleanup_items: list[tuple[str, str]], health_port: int) -> tuple[subprocess.Popen, float]:
    binary = repo_path / "target" / "release" / "torrust-tracker"
    env = os.environ.copy()
    env["TORRUST_TRACKER_CONFIG_TOML_PATH"] = str(config_path)
    log_file = log_path.open("w", encoding="utf-8")
    started_at = time.perf_counter()
    process = subprocess.Popen(
        [str(binary)],
        cwd=repo_path,
        env=env,
        stdout=log_file,
        stderr=subprocess.STDOUT,
        text=True,
    )
    cleanup_items.append(("process", str(process.pid)))
    wait_for_http_ok(f"http://127.0.0.1:{health_port}/health_check", 30)
    startup_ms = (time.perf_counter() - started_at) * 1000.0
    return process, startup_ms


def stop_tracker(process: subprocess.Popen, cleanup_items: list[tuple[str, str]]) -> None:
    if process.poll() is None:
        process.terminate()
        try:
            process.wait(timeout=10)
        except subprocess.TimeoutExpired:
            process.kill()
            process.wait(timeout=10)

    pid = str(process.pid)
    for index, item in enumerate(cleanup_items):
        if item == ("process", pid):
            cleanup_items.pop(index)
            break


def cleanup(workspace: Path | None, cleanup_items: list[tuple[str, str]]) -> None:
    while cleanup_items:
        kind, value = cleanup_items.pop()
        if kind == "container":
            subprocess.run(["docker", "rm", "-f", value], text=True, capture_output=True)
        elif kind == "process":
            try:
                os.kill(int(value), signal.SIGTERM)
            except ProcessLookupError:
                pass
    if workspace and workspace.exists():
        shutil.rmtree(workspace, ignore_errors=True)


class ThreadLocalHttpClient:
    def __init__(self, timeout: float = 5.0) -> None:
        self.timeout = timeout
        self.local = threading.local()

    def _connections(self) -> dict[tuple[str, str, int], http.client.HTTPConnection]:
        connections = getattr(self.local, "connections", None)
        if connections is None:
            connections = {}
            self.local.connections = connections
        return connections

    def request(self, method: str, url: str, headers: dict[str, str] | None = None, body: bytes | None = None) -> tuple[int, bytes]:
        parsed = urllib.parse.urlsplit(url)
        port = parsed.port or (443 if parsed.scheme == "https" else 80)
        path = parsed.path or "/"
        if parsed.query:
            path = f"{path}?{parsed.query}"
        key = (parsed.scheme, parsed.hostname or "127.0.0.1", port)
        connections = self._connections()
        connection = connections.get(key)

        if connection is None:
            if parsed.scheme == "https":
                connection = http.client.HTTPSConnection(key[1], key[2], timeout=self.timeout)
            else:
                connection = http.client.HTTPConnection(key[1], key[2], timeout=self.timeout)
            connections[key] = connection

        try:
            connection.request(method, path, body=body, headers=headers or {})
            response = connection.getresponse()
            payload = response.read()
            return response.status, payload
        except Exception:  # noqa: BLE001
            try:
                connection.close()
            except Exception:  # noqa: BLE001
                pass
            connections.pop(key, None)
            if parsed.scheme == "https":
                connection = http.client.HTTPSConnection(key[1], key[2], timeout=self.timeout)
            else:
                connection = http.client.HTTPConnection(key[1], key[2], timeout=self.timeout)
            connections[key] = connection
            connection.request(method, path, body=body, headers=headers or {})
            response = connection.getresponse()
            payload = response.read()
            return response.status, payload


def http_request(method: str, url: str, headers: dict[str, str] | None = None, body: bytes | None = None) -> tuple[int, bytes]:
    request = urllib.request.Request(url, data=body, headers=headers or {}, method=method)
    with urllib.request.urlopen(request, timeout=5) as response:
        return response.status, response.read()


def percentile(sorted_values: list[float], fraction: float) -> float:
    if not sorted_values:
        return 0.0
    index = max(0, math.ceil(len(sorted_values) * fraction) - 1)
    return sorted_values[index]


def summarize_latencies(latencies_ms: list[float], total_seconds: float) -> dict[str, float]:
    ordered = sorted(latencies_ms)
    return {
        "count": float(len(latencies_ms)),
        "total_ms": total_seconds * 1000.0,
        "ops_per_sec": len(latencies_ms) / total_seconds if total_seconds > 0 else 0.0,
        "mean_ms": statistics.fmean(latencies_ms) if latencies_ms else 0.0,
        "median_ms": statistics.median(latencies_ms) if latencies_ms else 0.0,
        "p95_ms": percentile(ordered, 0.95),
        "min_ms": ordered[0] if ordered else 0.0,
        "max_ms": ordered[-1] if ordered else 0.0,
    }


def benchmark_operations(label: str, functions: list[callable], concurrency: int) -> dict[str, object]:
    latencies_ms: list[float] = []
    started_at = time.perf_counter()

    if concurrency == 1:
        for operation in functions:
            op_started_at = time.perf_counter()
            operation()
            latencies_ms.append((time.perf_counter() - op_started_at) * 1000.0)
    else:
        with concurrent.futures.ThreadPoolExecutor(max_workers=concurrency) as executor:
            futures = []
            for operation in functions:
                futures.append(executor.submit(run_timed_operation, operation))
            for future in concurrent.futures.as_completed(futures):
                latencies_ms.append(future.result())

    total_seconds = time.perf_counter() - started_at
    return {
        "label": label,
        "stats": summarize_latencies(latencies_ms, total_seconds),
    }


def run_timed_operation(operation: callable) -> float:
    started_at = time.perf_counter()
    operation()
    return (time.perf_counter() - started_at) * 1000.0


def sha1_bytes(seed: str) -> bytes:
    return hashlib.sha1(seed.encode("utf-8")).digest()


def info_hash_bytes(prefix: str, index: int) -> bytes:
    return sha1_bytes(f"{prefix}-{index}")


def info_hash_hex(prefix: str, index: int) -> str:
    return info_hash_bytes(prefix, index).hex()


def peer_id_bytes(prefix: str, index: int) -> bytes:
    prefix_id = int.from_bytes(hashlib.sha1(prefix.encode("utf-8")).digest()[:2], "big") % 100
    return f"-CD{prefix_id:02d}-{index:014d}".encode("ascii")


def auth_key_value(prefix: str, index: int) -> str:
    return hashlib.sha256(f"{prefix}-{index}".encode("utf-8")).hexdigest()[:32]


def authorize_headers(content_type: str | None = None) -> dict[str, str]:
    headers = {"Authorization": f"Bearer {API_TOKEN}"}
    if content_type is not None:
        headers["Content-Type"] = content_type
    return headers


def assert_status(status: int, payload: bytes, expected: set[int], url: str) -> None:
    if status not in expected:
        snippet = payload.decode("utf-8", errors="replace")[:300]
        raise RuntimeError(f"Unexpected HTTP status {status} for {url}: {snippet}")


def post_whitelist(client: ThreadLocalHttpClient, api_origin: str, info_hash: str) -> None:
    url = f"{api_origin}/api/v1/whitelist/{info_hash}"
    status, payload = http_request("POST", url, headers=authorize_headers())
    assert_status(status, payload, {200, 201, 204}, url)


def reload_whitelist(client: ThreadLocalHttpClient, api_origin: str) -> None:
    url = f"{api_origin}/api/v1/whitelist/reload"
    status, payload = http_request("GET", url, headers=authorize_headers())
    assert_status(status, payload, {200}, url)


def post_auth_key(client: ThreadLocalHttpClient, api_origin: str, key: str) -> None:
    url = f"{api_origin}/api/v1/keys"
    body = json.dumps({"key": key, "seconds_valid": 3600}).encode("utf-8")
    status, payload = http_request("POST", url, headers=authorize_headers("application/json"), body=body)
    assert_status(status, payload, {200, 201, 204}, url)


def reload_keys(client: ThreadLocalHttpClient, api_origin: str) -> None:
    url = f"{api_origin}/api/v1/keys/reload"
    status, payload = http_request("GET", url, headers=authorize_headers())
    assert_status(status, payload, {200}, url)


def announce_started_then_completed(client: ThreadLocalHttpClient, tracker_origin: str, prefix: str, index: int) -> None:
    info_hash = info_hash_bytes(prefix, index)
    peer_id = peer_id_bytes(prefix, index)
    query_started = build_announce_query(info_hash, peer_id, 1, "started")
    query_completed = build_announce_query(info_hash, peer_id, 0, "completed")

    started_url = f"{tracker_origin}/announce?{query_started}"
    status, payload = http_request("GET", started_url)
    assert_status(status, payload, {200}, started_url)

    completed_url = f"{tracker_origin}/announce?{query_completed}"
    status, payload = http_request("GET", completed_url)
    assert_status(status, payload, {200}, completed_url)


def build_announce_query(info_hash: bytes, peer_id: bytes, left: int, event: str) -> str:
    return "&".join(
        [
            f"info_hash={urllib.parse.quote_from_bytes(info_hash)}",
            "peer_addr=192.168.1.88",
            f"peer_id={urllib.parse.quote_from_bytes(peer_id)}",
            "port=17548",
            "uploaded=0",
            "downloaded=0",
            f"left={left}",
            "compact=0",
            f"event={event}",
        ]
    )


def decode_bencode(data: bytes, offset: int = 0):
    token = data[offset : offset + 1]
    if token == b"i":
        end = data.index(b"e", offset)
        return int(data[offset + 1 : end]), end + 1
    if token == b"l":
        offset += 1
        items = []
        while data[offset : offset + 1] != b"e":
            item, offset = decode_bencode(data, offset)
            items.append(item)
        return items, offset + 1
    if token == b"d":
        offset += 1
        mapping = {}
        while data[offset : offset + 1] != b"e":
            key, offset = decode_bencode(data, offset)
            value, offset = decode_bencode(data, offset)
            mapping[key] = value
        return mapping, offset + 1
    if token.isdigit():
        colon = data.index(b":", offset)
        size = int(data[offset:colon])
        start = colon + 1
        end = start + size
        return data[start:end], end
    raise ValueError(f"Unexpected bencode token at offset {offset}: {token!r}")


def scrape_tracker(tracker_origin: str, info_hash: bytes) -> dict[bytes, object]:
    url = f"{tracker_origin}/scrape?info_hash={urllib.parse.quote_from_bytes(info_hash)}"
    with urllib.request.urlopen(url, timeout=5) as response:
        if response.status != 200:
            raise RuntimeError(f"Unexpected HTTP status {response.status} for {url}")
        payload = response.read()
    decoded, end_offset = decode_bencode(payload)
    if end_offset != len(payload):
        raise RuntimeError("Unexpected trailing bytes in scrape response")
    return decoded


def verify_persisted_download(client: ThreadLocalHttpClient, tracker_origin: str, prefix: str, index: int) -> None:
    info_hash = info_hash_bytes(prefix, index)
    deadline = time.time() + 10
    last_downloaded = None

    while time.time() < deadline:
        payload = scrape_tracker(tracker_origin, info_hash)
        files = payload.get(b"files")
        if isinstance(files, dict):
            stats = files.get(info_hash)
            if isinstance(stats, dict):
                last_downloaded = stats.get(b"downloaded")
                if last_downloaded == 1:
                    return
        time.sleep(0.25)

    raise RuntimeError(f"Expected downloaded=1 for {info_hash.hex()}, got {last_downloaded!r}")


def warm_up(client: ThreadLocalHttpClient, api_origin: str, tracker_origin: str) -> None:
    reload_whitelist(client, api_origin)
    reload_keys(client, api_origin)
    announce_started_then_completed(client, tracker_origin, "warmup-announce", 0)


def run_suite(
    variant: str,
    repo_path: Path,
    driver: str,
    mysql_version: str,
    postgres_version: str,
    ops: int,
    reload_iterations: int,
    concurrency: int,
) -> dict[str, object]:
    workspace = Path(tempfile.mkdtemp(prefix=f"torrust-benchmark-{variant}-{driver}-"))
    cleanup_items: list[tuple[str, str]] = []
    atexit.register(cleanup, workspace, cleanup_items)

    tracker_port = choose_free_port()
    api_port = choose_free_port()
    health_port = choose_free_port()
    database_path = start_database(driver, workspace, mysql_version, postgres_version, cleanup_items)
    config_path = write_tracker_config(workspace, driver, database_path, tracker_port, api_port, health_port)
    log_path = workspace / "tracker.log"

    process, startup_empty_ms = start_tracker(repo_path, config_path, log_path, cleanup_items, health_port)
    client = ThreadLocalHttpClient()
    api_origin = f"http://127.0.0.1:{api_port}"
    tracker_origin = f"http://127.0.0.1:{tracker_port}"

    try:
        warm_up(client, api_origin, tracker_origin)

        results: dict[str, object] = {
            "variant": variant,
            "repo_path": str(repo_path),
            "driver": driver,
            "startup_empty_ms": startup_empty_ms,
            "workloads": {},
            "log_path": str(log_path),
        }

        announce_seq_ops = [
            lambda index=index: announce_started_then_completed(client, tracker_origin, "announce-seq", index)
            for index in range(ops)
        ]
        results["workloads"]["announce_lifecycle_seq"] = benchmark_operations(
            "announce_lifecycle_seq", announce_seq_ops, 1
        )

        announce_conc_ops = [
            lambda index=index: announce_started_then_completed(client, tracker_origin, "announce-conc", index)
            for index in range(ops)
        ]
        results["workloads"]["announce_lifecycle_concurrent"] = benchmark_operations(
            "announce_lifecycle_concurrent", announce_conc_ops, concurrency
        )

        whitelist_seq_ops = [
            lambda index=index: post_whitelist(client, api_origin, info_hash_hex("whitelist-seq", index))
            for index in range(ops)
        ]
        results["workloads"]["whitelist_add_seq"] = benchmark_operations("whitelist_add_seq", whitelist_seq_ops, 1)

        whitelist_conc_ops = [
            lambda index=index: post_whitelist(client, api_origin, info_hash_hex("whitelist-conc", index))
            for index in range(ops)
        ]
        results["workloads"]["whitelist_add_concurrent"] = benchmark_operations(
            "whitelist_add_concurrent", whitelist_conc_ops, concurrency
        )

        whitelist_reload_ops = [lambda: reload_whitelist(client, api_origin) for _ in range(reload_iterations)]
        results["workloads"]["whitelist_reload"] = benchmark_operations("whitelist_reload", whitelist_reload_ops, 1)

        key_seq_ops = [
            lambda index=index: post_auth_key(client, api_origin, auth_key_value("bench-key-seq", index))
            for index in range(ops)
        ]
        results["workloads"]["auth_key_add_seq"] = benchmark_operations("auth_key_add_seq", key_seq_ops, 1)

        key_conc_ops = [
            lambda index=index: post_auth_key(client, api_origin, auth_key_value("bench-key-conc", index))
            for index in range(ops)
        ]
        results["workloads"]["auth_key_add_concurrent"] = benchmark_operations(
            "auth_key_add_concurrent", key_conc_ops, concurrency
        )

        key_reload_ops = [lambda: reload_keys(client, api_origin) for _ in range(reload_iterations)]
        results["workloads"]["auth_key_reload"] = benchmark_operations("auth_key_reload", key_reload_ops, 1)

        stop_tracker(process, cleanup_items)
        process, startup_populated_ms = start_tracker(repo_path, config_path, log_path, cleanup_items, health_port)
        results["startup_populated_ms"] = startup_populated_ms
        reload_whitelist(client, api_origin)
        reload_keys(client, api_origin)

        stop_tracker(process, cleanup_items)
        return results
    finally:
        if process.poll() is None:
            stop_tracker(process, cleanup_items)
        cleanup(workspace, cleanup_items)


def compare_results(results: list[dict[str, object]]) -> list[dict[str, object]]:
    indexed = {(entry["driver"], entry["variant"]): entry for entry in results}
    comparisons: list[dict[str, object]] = []

    for driver in sorted({entry["driver"] for entry in results}):
        before = indexed[(driver, "before")]
        after = indexed[(driver, "after")]
        driver_comparison: dict[str, object] = {
            "driver": driver,
            "startup_empty_speedup": before["startup_empty_ms"] / after["startup_empty_ms"],
            "startup_populated_speedup": before["startup_populated_ms"] / after["startup_populated_ms"],
            "workloads": {},
        }

        before_workloads = before["workloads"]
        after_workloads = after["workloads"]
        for workload_name in before_workloads:
            before_ops = before_workloads[workload_name]["stats"]["ops_per_sec"]
            after_ops = after_workloads[workload_name]["stats"]["ops_per_sec"]
            before_p95 = before_workloads[workload_name]["stats"]["p95_ms"]
            after_p95 = after_workloads[workload_name]["stats"]["p95_ms"]
            driver_comparison["workloads"][workload_name] = {
                "ops_per_sec_speedup": after_ops / before_ops if before_ops else 0.0,
                "p95_latency_improvement": before_p95 / after_p95 if after_p95 else 0.0,
            }
        comparisons.append(driver_comparison)

    return comparisons


def print_summary(results: list[dict[str, object]], comparisons: list[dict[str, object]]) -> None:
    print("")
    print("Startup")
    print("variant   driver       empty_ms  populated_ms")
    for entry in results:
        print(
            f"{entry['variant']:<8} {entry['driver']:<11} "
            f"{entry['startup_empty_ms']:>8.1f} {entry['startup_populated_ms']:>13.1f}"
        )

    print("")
    print("Workloads (ops/s, p95 ms)")
    print("variant   driver       workload                        ops_per_sec      p95_ms")
    for entry in results:
        for workload_name, workload in entry["workloads"].items():
            print(
                f"{entry['variant']:<8} {entry['driver']:<11} {workload_name:<30} "
                f"{workload['stats']['ops_per_sec']:>11.2f} {workload['stats']['p95_ms']:>11.2f}"
            )

    print("")
    print("After vs Before Speedup")
    print("driver       workload                        ops/s_x   p95_improvement_x")
    for driver_comparison in comparisons:
        for workload_name, workload in driver_comparison["workloads"].items():
            print(
                f"{driver_comparison['driver']:<11} {workload_name:<30} "
                f"{workload['ops_per_sec_speedup']:>7.2f} {workload['p95_latency_improvement']:>18.2f}"
            )


def main() -> int:
    args = parse_args()

    if not args.skip_build:
        build_tracker_binary(args.before_repo)
        build_tracker_binary(args.after_repo)

    results: list[dict[str, object]] = []
    suites = [("before", args.before_repo), ("after", args.after_repo)]
    for driver in args.dbs:
        for variant, repo_path in suites:
            print(f"Running {variant} benchmark on {driver}...", flush=True)
            result = run_suite(
                variant=variant,
                repo_path=repo_path,
                driver=driver,
                mysql_version=args.mysql_version,
                postgres_version=args.postgres_version,
                ops=args.ops,
                reload_iterations=args.reload_iterations,
                concurrency=args.concurrency,
            )
            results.append(result)

    comparisons = compare_results(results)
    payload = {"results": results, "comparisons": comparisons}

    if args.json_output is not None:
        args.json_output.write_text(json.dumps(payload, indent=2), encoding="utf-8")

    print_summary(results, comparisons)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
