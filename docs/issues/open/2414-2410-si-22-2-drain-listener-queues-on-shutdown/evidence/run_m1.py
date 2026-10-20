import subprocess, time, signal, os, urllib.request, datetime, sqlite3, re

def now(): return datetime.datetime.now(datetime.timezone.utc).strftime("%H:%M:%S.%f")[:-3]

root = os.getcwd()
log = open(".tmp/m1/tracker.log", "w")
for f in (".tmp/m1/tracker.sqlite3", ".tmp/m1/tracker.sqlite3-wal", ".tmp/m1/tracker.sqlite3-shm"):
    if os.path.exists(f): os.remove(f)
env = {k: v for k, v in os.environ.items() if k not in ("TORRUST_TRACKER_CONFIG_TOML", "TORRUST_TRACKER_CONFIG_TOML_PATH")}
tracker = subprocess.Popen(["./target/debug/torrust-tracker", "-c", ".tmp/m1/tracker.toml"], stdout=log, stderr=subprocess.STDOUT, env=env)
ready = False
for _ in range(150):
    time.sleep(0.2)
    try:
        urllib.request.urlopen("http://127.0.0.1:47413/health_check", timeout=1).read()
        if "shutdown signal handlers installed" in open(".tmp/m1/tracker.log").read():
            ready = True; break
    except Exception: pass
print("ready", ready, "at", now())
ih = "3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0"
def udp(event, i):
    subprocess.run(["./target/debug/tracker_client", "udp", "announce", "127.0.0.1:47171", ih, "--event", event, "--port", str(20000 + i), "--peer-id=-qB0000-0000000000%02d" % i], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
def http(event, p):
    url = "http://127.0.0.1:47170/announce?info_hash=SI22SI22SI22SI22SI22&peer_id=-SI2200-%012d&port=%d&uploaded=0&downloaded=0&left=0&event=%s" % (p, 30000 + p, event)
    try: urllib.request.urlopen(url, timeout=5).read()
    except Exception as e: print("http failure", event, p, e)
for i in range(1, 61): udp("started", i)
for i in range(1, 61): udp("completed", i)
for p in range(1, 101): http("started", p)
for p in range(1, 101): http("completed", p)
print("announces sent, SIGTERM at", now())
tracker.send_signal(signal.SIGTERM)
code = tracker.wait(timeout=60)
print("tracker_exit_status=%s at %s" % (code, now()))
log.close()
con = sqlite3.connect(".tmp/m1/tracker.sqlite3")
print("torrents:", con.execute("select hex(info_hash), completed from torrents").fetchall() if False else con.execute("select * from torrents").fetchall())
print("aggregate:", con.execute("select * from torrent_aggregate_metrics").fetchall())
