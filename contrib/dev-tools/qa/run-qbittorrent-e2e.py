#!/usr/bin/env python3

import argparse
import atexit
import base64
import hashlib
import json
import os
import shutil
import signal
import socket
import subprocess
import sys
import tempfile
import time
import urllib.error
import urllib.parse
import urllib.request
from pathlib import Path


ROOT_DIR = Path(__file__).resolve().parents[3]
TRACKER_BINARY = ROOT_DIR / "target" / "debug" / "torrust-tracker"
DEFAULT_QBITTORRENT_IMAGE = "qbittorrentofficial/qbittorrent-nox:latest"
QBITTORRENT_PASSWORD = "codex-pass"
QBITTORRENT_USERNAME = "admin"


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Run a qBittorrent end-to-end test against the tracker.")
    parser.add_argument("--db-driver", choices=("sqlite3", "mysql", "postgresql"), default="postgresql")
    parser.add_argument("--protocol", choices=("http", "udp"), default="http")
    parser.add_argument("--mysql-version", default="8.4")
    parser.add_argument("--postgres-version", default="16")
    parser.add_argument("--qbittorrent-image", default=DEFAULT_QBITTORRENT_IMAGE)
    parser.add_argument("--timeout-seconds", type=int, default=180)
    parser.add_argument("--keep-artifacts", action="store_true")
    return parser.parse_args()


def run_command(*args: str, cwd: Path | None = None, env: dict[str, str] | None = None, check: bool = True) -> subprocess.CompletedProcess:
    return subprocess.run(
        args,
        cwd=cwd or ROOT_DIR,
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
            with urllib.request.urlopen(url, timeout=3) as response:
                if response.status == 200:
                    return
        except Exception as err:  # noqa: BLE001
            last_error = err
        time.sleep(1)
    raise RuntimeError(f"Timed out waiting for {url!r}: {last_error}")


def docker_exec(container: str, *args: str, check: bool = True) -> subprocess.CompletedProcess:
    return run_command("docker", "exec", container, *args, check=check)


def docker_port(container: str, container_port: int, protocol: str = "tcp") -> int:
    result = run_command("docker", "port", container, f"{container_port}/{protocol}")
    _, port = result.stdout.strip().rsplit(":", 1)
    return int(port)


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


def pbkdf2_password(password: str) -> str:
    salt = os.urandom(16)
    digest = hashlib.pbkdf2_hmac("sha512", password.encode(), salt, 100_000)
    return f"@ByteArray({base64.b64encode(salt).decode()}:{base64.b64encode(digest).decode()})"


def write_qbittorrent_config(config_root: Path, peer_port: int) -> None:
    config_dir = config_root / "qBittorrent" / "config"
    config_dir.mkdir(parents=True, exist_ok=True)
    config = (
        "[BitTorrent]\n"
        "Session\\AddTorrentStopped=false\n"
        "Session\\DefaultSavePath=/downloads\n"
        f"Session\\Port={peer_port}\n"
        "Session\\TempPath=/downloads/temp\n"
        "[Preferences]\n"
        "WebUI\\LocalHostAuth=false\n"
        "WebUI\\Port=8080\n"
        f'WebUI\\Password_PBKDF2="{pbkdf2_password(QBITTORRENT_PASSWORD)}"\n'
        f"WebUI\\Username={QBITTORRENT_USERNAME}\n"
    )
    (config_dir / "qBittorrent.conf").write_text(config, encoding="utf-8")


def bencode(value) -> bytes:
    if isinstance(value, int):
        return f"i{value}e".encode()
    if isinstance(value, bytes):
        return str(len(value)).encode() + b":" + value
    if isinstance(value, str):
        return bencode(value.encode())
    if isinstance(value, list):
        return b"l" + b"".join(bencode(item) for item in value) + b"e"
    if isinstance(value, dict):
        encoded_items = []
        for key in sorted(value):
            encoded_items.append(bencode(key))
            encoded_items.append(bencode(value[key]))
        return b"d" + b"".join(encoded_items) + b"e"
    raise TypeError(f"Unsupported bencode type: {type(value)!r}")


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


def build_torrent(payload_path: Path, torrent_path: Path, announce_url: str) -> bytes:
    payload = payload_path.read_bytes()
    piece_length = 16 * 1024
    pieces = b"".join(hashlib.sha1(payload[index : index + piece_length]).digest() for index in range(0, len(payload), piece_length))
    info = {
        b"length": len(payload),
        b"name": payload_path.name.encode(),
        b"piece length": piece_length,
        b"pieces": pieces,
    }
    torrent = {
        b"announce": announce_url.encode(),
        b"created by": b"codex-qb-e2e",
        b"creation date": int(time.time()),
        b"info": info,
    }
    torrent_path.write_bytes(bencode(torrent))
    return hashlib.sha1(bencode(info)).digest()


def build_tracker_binary() -> None:
    run_command("cargo", "build", "--bin", "torrust-tracker")


def start_database(driver: str, workspace: Path, args: argparse.Namespace, cleanup_items: list[tuple[str, str]]):
    if driver == "sqlite3":
        db_path = workspace / "tracker.sqlite3.db"
        return str(db_path)

    if driver == "mysql":
        host_port = choose_free_port()
        container = f"torrust-mysql-e2e-{os.getpid()}"
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
            f"mysql:{args.mysql_version}",
            "--default-authentication-plugin=mysql_native_password",
        )
        cleanup_items.append(("container", container))
        wait_for_mysql(container, 60)
        return f"mysql://root:test@127.0.0.1:{host_port}/torrust_tracker"

    host_port = choose_free_port()
    container = f"torrust-postgres-e2e-{os.getpid()}"
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
        f"postgres:{args.postgres_version}",
    )
    cleanup_items.append(("container", container))
    wait_for_postgres(container, 60)
    return f"postgresql://postgres:test@127.0.0.1:{host_port}/torrust_tracker"


def write_tracker_config(workspace: Path, driver: str, database_path: str, http_port: int, udp_port: int, health_port: int) -> Path:
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
                'threshold = "debug"',
                "",
                "[core]",
                "listed = false",
                "private = false",
                "",
                "[core.database]",
                f'driver = "{driver}"',
                f'path = "{database_path}"',
                "",
                "[[udp_trackers]]",
                f'bind_address = "0.0.0.0:{udp_port}"',
                "",
                "[[http_trackers]]",
                f'bind_address = "0.0.0.0:{http_port}"',
                "",
                "[health_check_api]",
                f'bind_address = "127.0.0.1:{health_port}"',
                "",
            ]
        ),
        encoding="utf-8",
    )
    return config_path


def start_tracker(config_path: Path, log_path: Path, cleanup_items: list[tuple[str, str]], health_port: int) -> subprocess.Popen:
    env = os.environ.copy()
    env["TORRUST_TRACKER_CONFIG_TOML_PATH"] = str(config_path)
    log_file = log_path.open("w", encoding="utf-8")
    process = subprocess.Popen(
        [str(TRACKER_BINARY)],
        cwd=ROOT_DIR,
        env=env,
        stdout=log_file,
        stderr=subprocess.STDOUT,
        text=True,
    )
    cleanup_items.append(("process", str(process.pid)))
    wait_for_http_ok(f"http://127.0.0.1:{health_port}/health_check", 30)
    return process


def qb_login(container: str) -> None:
    result = docker_exec(
        container,
        "curl",
        "-s",
        "-c",
        "/tmp/qb.cookies",
        "--data",
        f"username={QBITTORRENT_USERNAME}&password={QBITTORRENT_PASSWORD}",
        "http://127.0.0.1:8080/api/v2/auth/login",
    )
    if result.stdout.strip() != "Ok.":
        raise RuntimeError(f"Unable to login to qBittorrent container {container!r}: {result.stdout!r} {result.stderr!r}")


def qb_get_json(container: str, path: str) -> object:
    qb_login(container)
    result = docker_exec(
        container,
        "curl",
        "-s",
        "-b",
        "/tmp/qb.cookies",
        f"http://127.0.0.1:8080{path}",
    )
    return json.loads(result.stdout)


def qb_get_text(container: str, path: str) -> str:
    qb_login(container)
    result = docker_exec(
        container,
        "curl",
        "-s",
        "-b",
        "/tmp/qb.cookies",
        f"http://127.0.0.1:8080{path}",
    )
    return result.stdout.strip()


def qb_post_form(container: str, path: str, args: list[str]) -> str:
    qb_login(container)
    result = docker_exec(
        container,
        "curl",
        "-s",
        "-b",
        "/tmp/qb.cookies",
        *args,
        f"http://127.0.0.1:8080{path}",
    )
    return result.stdout


def wait_for_qb_api(container: str, timeout_seconds: int) -> None:
    deadline = time.time() + timeout_seconds
    while time.time() < deadline:
        try:
            version = qb_get_text(container, "/api/v2/app/version")
            if version.startswith("v"):
                return
        except Exception:  # noqa: BLE001
            pass
        time.sleep(1)
    raise RuntimeError(f"Timed out waiting for qBittorrent API in {container!r}")


def start_qbittorrent(name: str, image: str, config_root: Path, downloads_root: Path, shared_root: Path, peer_port: int, cleanup_items: list[tuple[str, str]]) -> str:
    subprocess.run(["docker", "rm", "-f", name], text=True, capture_output=True)
    run_command(
        "docker",
        "run",
        "-d",
        "--rm",
        "--name",
        name,
        "-e",
        "QBT_LEGAL_NOTICE=confirm",
        "-e",
        f"QBT_TORRENTING_PORT={peer_port}",
        "-v",
        f"{config_root}:/config",
        "-v",
        f"{downloads_root}:/downloads",
        "-v",
        f"{shared_root}:/shared",
        "-p",
        f"{peer_port}:{peer_port}",
        "-p",
        f"{peer_port}:{peer_port}/udp",
        image,
    )
    cleanup_items.append(("container", name))
    wait_for_qb_api(name, 30)
    return name


def qb_add_torrent(container: str, torrent_path_in_container: str) -> None:
    qb_post_form(
        container,
        "/api/v2/torrents/add",
        [
            "-F",
            f"torrents=@{torrent_path_in_container}",
            "-F",
            "savepath=/downloads",
            "-F",
            "paused=false",
            "-F",
            "skip_checking=false",
        ],
    )


def qb_torrent_info(container: str, info_hash_hex: str) -> dict:
    torrents = qb_get_json(container, f"/api/v2/torrents/info?hashes={info_hash_hex}")
    if not torrents:
        raise RuntimeError(f"Torrent {info_hash_hex} not found in {container!r}")
    return torrents[0]


def qb_trackers(container: str, info_hash_hex: str) -> list[dict]:
    trackers = qb_get_json(container, f"/api/v2/torrents/trackers?hash={info_hash_hex}")
    return [tracker for tracker in trackers if isinstance(tracker, dict) and tracker.get("url")]


def wait_for_progress(container: str, info_hash_hex: str, expected_progress: float, timeout_seconds: int) -> dict:
    deadline = time.time() + timeout_seconds
    last_info = None
    while time.time() < deadline:
        last_info = qb_torrent_info(container, info_hash_hex)
        if float(last_info.get("progress", 0.0)) >= expected_progress:
            return last_info
        time.sleep(1)
    raise RuntimeError(f"Timed out waiting for progress {expected_progress} in {container!r}: {last_info}")


def wait_for_tracker_contact(container: str, info_hash_hex: str, timeout_seconds: int) -> list[dict]:
    deadline = time.time() + timeout_seconds
    last_trackers = None
    while time.time() < deadline:
        last_trackers = qb_trackers(container, info_hash_hex)
        if any(tracker.get("status", 0) != 0 for tracker in last_trackers):
            return last_trackers
        time.sleep(1)
    raise RuntimeError(f"Timed out waiting for tracker contact in {container!r}: {last_trackers}")


def scrape_tracker(http_port: int, info_hash: bytes) -> dict[bytes, object]:
    encoded_info_hash = urllib.parse.quote_from_bytes(info_hash)
    url = f"http://127.0.0.1:{http_port}/scrape?info_hash={encoded_info_hash}"
    with urllib.request.urlopen(url, timeout=10) as response:
        payload = response.read()
    decoded, end_offset = decode_bencode(payload)
    if end_offset != len(payload):
        raise RuntimeError("Unexpected trailing bytes in scrape response")
    return decoded


def cleanup(workspace: Path | None, keep_artifacts: bool, cleanup_items: list[tuple[str, str]]) -> None:
    while cleanup_items:
        kind, value = cleanup_items.pop()
        if kind == "container":
            subprocess.run(["docker", "rm", "-f", value], text=True, capture_output=True)
        elif kind == "process":
            try:
                os.kill(int(value), signal.SIGTERM)
            except ProcessLookupError:
                pass
    if workspace and workspace.exists() and not keep_artifacts:
        shutil.rmtree(workspace, ignore_errors=True)


def main() -> int:
    args = parse_args()
    workspace = Path(tempfile.mkdtemp(prefix="torrust-qb-e2e-"))
    cleanup_items: list[tuple[str, str]] = []
    atexit.register(cleanup, workspace, args.keep_artifacts, cleanup_items)

    http_port = choose_free_port()
    udp_port = choose_free_port()
    health_port = choose_free_port()
    seeder_peer_port = choose_free_port()
    leecher_peer_port = choose_free_port()

    shared_root = workspace / "shared"
    seeder_downloads = workspace / "seeder-downloads"
    leecher_downloads = workspace / "leecher-downloads"
    seeder_config = workspace / "seeder-config"
    leecher_config = workspace / "leecher-config"

    shared_root.mkdir(parents=True, exist_ok=True)
    seeder_downloads.mkdir(parents=True, exist_ok=True)
    leecher_downloads.mkdir(parents=True, exist_ok=True)

    write_qbittorrent_config(seeder_config, seeder_peer_port)
    write_qbittorrent_config(leecher_config, leecher_peer_port)

    payload_path = shared_root / "payload.bin"
    payload_path.write_bytes(os.urandom(256 * 1024))
    shutil.copy2(payload_path, seeder_downloads / payload_path.name)

    tracker_scheme = "http" if args.protocol == "http" else "udp"
    tracker_port = http_port if args.protocol == "http" else udp_port
    torrent_path = shared_root / "payload.torrent"
    info_hash = build_torrent(payload_path, torrent_path, f"{tracker_scheme}://host.docker.internal:{tracker_port}/announce")
    info_hash_hex = info_hash.hex()

    build_tracker_binary()
    database_path = start_database(args.db_driver, workspace, args, cleanup_items)
    tracker_config = write_tracker_config(workspace, args.db_driver, database_path, http_port, udp_port, health_port)
    tracker_log = workspace / "tracker.log"
    tracker_process = start_tracker(tracker_config, tracker_log, cleanup_items, health_port)

    start_qbittorrent("torrust-qb-seeder", args.qbittorrent_image, seeder_config, seeder_downloads, shared_root, seeder_peer_port, cleanup_items)
    start_qbittorrent("torrust-qb-leecher", args.qbittorrent_image, leecher_config, leecher_downloads, shared_root, leecher_peer_port, cleanup_items)

    qb_add_torrent("torrust-qb-seeder", "/shared/payload.torrent")
    seeder_info = wait_for_progress("torrust-qb-seeder", info_hash_hex, 1.0, args.timeout_seconds)
    seeder_trackers = wait_for_tracker_contact("torrust-qb-seeder", info_hash_hex, args.timeout_seconds)

    qb_add_torrent("torrust-qb-leecher", "/shared/payload.torrent")
    leecher_info = wait_for_progress("torrust-qb-leecher", info_hash_hex, 1.0, args.timeout_seconds)
    leecher_trackers = wait_for_tracker_contact("torrust-qb-leecher", info_hash_hex, args.timeout_seconds)

    downloaded_path = leecher_downloads / payload_path.name
    if not downloaded_path.exists():
        raise RuntimeError(f"Leecher did not create the expected payload file: {downloaded_path}")
    if downloaded_path.read_bytes() != payload_path.read_bytes():
        raise RuntimeError("Leecher payload does not match the seeded payload")

    scrape_response = scrape_tracker(http_port, info_hash)
    files_section = scrape_response[b"files"]
    torrent_stats = files_section[info_hash]
    if torrent_stats[b"complete"] < 1:
        raise RuntimeError(f"Unexpected scrape complete count: {torrent_stats}")
    if torrent_stats[b"downloaded"] < 1:
        raise RuntimeError(f"Unexpected scrape downloaded count: {torrent_stats}")

    print("Seeder info:", json.dumps(seeder_info, indent=2, sort_keys=True))
    print("Seeder trackers:", json.dumps(seeder_trackers, indent=2, sort_keys=True))
    print("Leecher info:", json.dumps(leecher_info, indent=2, sort_keys=True))
    print("Leecher trackers:", json.dumps(leecher_trackers, indent=2, sort_keys=True))
    print(
        "Tracker scrape stats:",
        json.dumps(
            {
                "complete": torrent_stats[b"complete"],
                "downloaded": torrent_stats[b"downloaded"],
                "incomplete": torrent_stats[b"incomplete"],
            },
            indent=2,
            sort_keys=True,
        ),
    )
    print(f"Tracker log: {tracker_log}")

    tracker_process.terminate()
    try:
        tracker_process.wait(timeout=10)
    except subprocess.TimeoutExpired:
        tracker_process.kill()
        tracker_process.wait(timeout=10)

    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except Exception as err:  # noqa: BLE001
        print(err, file=sys.stderr)
        raise
