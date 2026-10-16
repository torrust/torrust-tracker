"""Scenario M1 sender for #2244 (the script that produced the V5 output).

Sends 1,000 sequential BEP 15 connect requests to a local tracker started with
`.tmp/2244/tracker-info.toml`, then reads the statistics endpoint. It was run
inline (`python3 - <<'EOF'`) and is saved here verbatim.
"""
import random, socket, struct, time, json, urllib.request
s=socket.socket(socket.AF_INET, socket.SOCK_DGRAM); s.settimeout(2)
for _ in range(1000):
    tx=random.getrandbits(32); s.sendto(struct.pack(">QII",0x41727101980,0,tx),("127.0.0.1",26969)); s.recvfrom(64)
time.sleep(2)
d=json.load(urllib.request.urlopen("http://127.0.0.1:21212/api/v1/stats?token=MyAccessToken"))
print("sent 1000 UDP connect requests over IPv4")
for k in ["udp4_requests","udp4_connections_handled","udp_requests_aborted","udp_requests_banned","udp_banned_ips_total","udp_avg_connect_processing_time_ns","udp_avg_announce_processing_time_ns","udp_avg_scrape_processing_time_ns","udp4_responses","udp4_errors_handled","tcp4_announces_handled"]:
    print(f"{k}: {d.get(k)!r}")
