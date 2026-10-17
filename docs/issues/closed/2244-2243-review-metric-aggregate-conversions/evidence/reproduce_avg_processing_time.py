"""Reproduce the UDP average processing time bias (#2244, G3).

Sends N sequential BEP 15 connect requests to a local tracker, then compares
the REST API's `udp_avg_connect_processing_time_ns` with the true mean of the
per-request processing times the tracker logged at debug level.
"""
import json
import os
import random
import re
import socket
import struct
import sys
import time
import urllib.request

UDP_ADDR = ("127.0.0.1", 26969)
API = "http://127.0.0.1:21212/api/v1/stats?token=MyAccessToken"
PROTOCOL_ID = 0x41727101980


def send_connects(n):
    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    sock.settimeout(2)
    for _ in range(n):
        tx = random.getrandbits(32)
        sock.sendto(struct.pack(">QII", PROTOCOL_ID, 0, tx), UDP_ADDR)
        data, _ = sock.recvfrom(64)
        action, rtx, _cid = struct.unpack(">IIQ", data[:16])
        assert action == 0 and rtx == tx, "unexpected connect response"


def true_mean_from_log(log_path):
    pattern = re.compile(r'request_kind.*?connect.*?req_processing_time: ([0-9.e+]+) ns')
    times = []
    with open(log_path, encoding="utf-8", errors="replace") as log:
        for line in log:
            if "Recalculated UDP average processing time" in line:
                match = pattern.search(line)
                if match:
                    times.append(float(match.group(1)))
    return times


def wait_until_log_settles(log_path):
    """The statistics handler runs behind the request path; wait until it stops logging."""
    previous = -1
    while True:
        size = os.path.getsize(log_path)
        if size == previous:
            return
        previous = size
        time.sleep(2)


def main():
    n = int(sys.argv[1])
    log_path = sys.argv[2]
    send_connects(n)
    wait_until_log_settles(log_path)
    stats = json.load(urllib.request.urlopen(API))
    reported = stats["udp_avg_connect_processing_time_ns"]
    times = true_mean_from_log(log_path)
    print(f"requests sent:                       {n}")
    print(f"processing times logged (averaged):  {len(times)}")
    print(f"min / max logged processing time:    {min(times):.0f} / {max(times):.0f} ns")
    print(f"true mean of logged times:           {sum(times) / len(times):.1f} ns")
    print(f"API udp_avg_connect_processing_time_ns: {reported} ns")


if __name__ == "__main__":
    main()
