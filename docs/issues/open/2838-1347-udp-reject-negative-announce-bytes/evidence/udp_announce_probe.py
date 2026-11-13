"""Disposable probe: send one UDP connect + one announce with a chosen signed `left`.

Usage: python3 -I udp_announce_probe.py <port> <left> [uploaded] [downloaded]
Prints the announce response (action, seeders, leechers) or the error message.
Exits with status 1, without printing a result, when a response does not carry the
transaction id of its request. Also exits with status 1, printing the decoded error
message, when the connect response's action is not 0.
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

CONNECT_TID, ANNOUNCE_TID = 1234, 4321


def check_transaction_id(name, sent, received):
    if received != sent:
        sys.exit(f"{name}: transaction id mismatch: sent {sent}, received {received}")


s.sendto(struct.pack(">qii", 0x41727101980, 0, CONNECT_TID), addr)
connect_response = s.recvfrom(64)[0]
action, tid = struct.unpack(">ii", connect_response[:8])
check_transaction_id("connect", CONNECT_TID, tid)
if action != 0:
    sys.exit(f"connect: action={action} message={connect_response[8:].decode(errors='replace')!r}")
(connection_id,) = struct.unpack(">q", connect_response[8:16])

packet = struct.pack(
    ">qii20s20sqqqiIiiH",  # cspell:disable-line
    connection_id, 1, ANNOUNCE_TID,
    b"\x11" * 20, b"-XX0001-000000000000",
    downloaded, left, uploaded,
    2,  # event: started
    0, 7, -1, 6881,
)
s.sendto(packet, addr)
data = s.recvfrom(2048)[0]
action, tid = struct.unpack(">ii", data[:8])
check_transaction_id("announce", ANNOUNCE_TID, tid)
if action == 1:
    interval, leechers, seeders = struct.unpack(">iii", data[8:20])
    print(f"announce response: interval={interval} leechers={leechers} seeders={seeders}")
else:
    print(f"action={action} message={data[8:].decode(errors='replace')!r}")
