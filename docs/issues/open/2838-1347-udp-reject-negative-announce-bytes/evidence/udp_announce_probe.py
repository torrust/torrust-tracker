"""Disposable probe: send one UDP connect + one announce with a chosen signed `left`.

Usage: python3 -I udp_announce_probe.py <port> <left> [uploaded] [downloaded]
Prints the announce response (action, seeders, leechers) or the error message.
"""
import socket
import struct
import sys

port, left = int(sys.argv[1]), int(sys.argv[2])
uploaded = int(sys.argv[3]) if len(sys.argv) > 3 else 0
downloaded = int(sys.argv[4]) if len(sys.argv) > 4 else 0

s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
s.settimeout(3)
addr = ("127.0.0.1", port)

s.sendto(struct.pack(">qii", 0x41727101980, 0, 1234), addr)
action, tid, connection_id = struct.unpack(">iiq", s.recvfrom(64)[0][:16])

packet = struct.pack(
    ">qii20s20sqqqiIiiH",  # cspell:disable-line
    connection_id, 1, 4321,
    b"\x11" * 20, b"-XX0001-000000000000",
    downloaded, left, uploaded,
    2,  # event: started
    0, 7, -1, 6881,
)
s.sendto(packet, addr)
data = s.recvfrom(2048)[0]
action, tid = struct.unpack(">ii", data[:8])
if action == 1:
    interval, leechers, seeders = struct.unpack(">iii", data[8:20])
    print(f"announce response: interval={interval} leechers={leechers} seeders={seeders}")
else:
    print(f"action={action} message={data[8:].decode(errors='replace')!r}")
