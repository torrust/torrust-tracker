"""Disposable M1 harness for #2416: SIGTERM during a mixed HTTP and UDP completed-download burst.

Run from the repository root with .tmp/m1/tracker.toml in place (a copy of m1-tracker.toml).
Environment: UDP_PEERS, HTTP_PEERS, SIGTERM_DELAY (seconds into the burst), WORKERS.
"""
import datetime
import os
import signal
import sqlite3
import subprocess
import threading
import time
import urllib.request
from concurrent.futures import ThreadPoolExecutor

UDP_PEERS = int(os.environ.get("UDP_PEERS", "150"))
HTTP_PEERS = int(os.environ.get("HTTP_PEERS", "500"))
SIGTERM_DELAY = float(os.environ.get("SIGTERM_DELAY", "0.3"))
WORKERS = int(os.environ.get("WORKERS", "48"))
UDP_INFO_HASH = "3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0"


def now():
    return datetime.datetime.now(datetime.timezone.utc).strftime("%H:%M:%S.%f")[:-3]


log = open(".tmp/m1/tracker.log", "w")
for path in (".tmp/m1/tracker.sqlite3", ".tmp/m1/tracker.sqlite3-wal", ".tmp/m1/tracker.sqlite3-shm"):
    if os.path.exists(path):
        os.remove(path)
env = {k: v for k, v in os.environ.items() if k not in ("TORRUST_TRACKER_CONFIG_TOML", "TORRUST_TRACKER_CONFIG_TOML_PATH")}
tracker = subprocess.Popen(
    ["./target/debug/torrust-tracker", "-c", ".tmp/m1/tracker.toml"], stdout=log, stderr=subprocess.STDOUT, env=env
)
ready = False
for _ in range(150):
    time.sleep(0.2)
    try:
        urllib.request.urlopen("http://127.0.0.1:47513/health_check", timeout=1).read()
        if "shutdown signal handlers installed" in open(".tmp/m1/tracker.log").read():
            ready = True
            break
    except Exception:
        pass
print("ready", ready, "at", now())


def udp(event, i):
    result = subprocess.run(
        ["./target/debug/tracker_client", "udp", "announce", "127.0.0.1:47271", UDP_INFO_HASH, "--event", event,
         "--port", str(20000 + i), "--peer-id=-qB0000-00000000%04d" % i],
        capture_output=True, text=True, timeout=30,
    )
    return result.returncode == 0 and "AnnounceIpv" in result.stdout


def http(event, p):
    url = ("http://127.0.0.1:47270/announce?info_hash=SI22SI22SI22SI22SI22&peer_id=-SI2200-%012d&port=%d"
           "&uploaded=0&downloaded=0&left=0&event=%s" % (p, 30000 + p, event))
    try:
        return b"8:interval" in urllib.request.urlopen(url, timeout=5).read()
    except Exception:
        return False


with ThreadPoolExecutor(WORKERS) as pool:
    started = list(pool.map(lambda i: udp("started", i), range(1, UDP_PEERS + 1))) + list(
        pool.map(lambda p: http("started", p), range(1, HTTP_PEERS + 1))
    )
print("started announces answered: %d of %d" % (sum(started), len(started)))

jobs = [("udp", i) for i in range(1, UDP_PEERS + 1)] + [("http", p) for p in range(1, HTTP_PEERS + 1)]
sigterm_at = []


def send_sigterm():
    time.sleep(SIGTERM_DELAY)
    sigterm_at.append(now())
    tracker.send_signal(signal.SIGTERM)


timer = threading.Thread(target=send_sigterm)
burst_started = now()
timer.start()
with ThreadPoolExecutor(WORKERS) as pool:
    answered = list(pool.map(lambda job: (job[0], udp("completed", job[1]) if job[0] == "udp" else http("completed", job[1])), jobs))
timer.join()
code = tracker.wait(timeout=60)
print("burst started %s, SIGTERM at %s, tracker_exit_status=%s at %s" % (burst_started, sigterm_at[0], code, now()))
log.close()
udp_answered = sum(1 for kind, ok in answered if kind == "udp" and ok)
http_answered = sum(1 for kind, ok in answered if kind == "http" and ok)
print("completed answered: udp %d of %d, http %d of %d" % (udp_answered, UDP_PEERS, http_answered, HTTP_PEERS))
con = sqlite3.connect(".tmp/m1/tracker.sqlite3")
print("torrents:", con.execute("select * from torrents").fetchall())
print("aggregate:", con.execute("select * from torrent_aggregate_metrics").fetchall())
