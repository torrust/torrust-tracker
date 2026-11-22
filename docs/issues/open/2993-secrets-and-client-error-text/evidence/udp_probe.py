# Usage: python3 -I udp_probe.py <port>. Sends a BEP 15 connect, then an announce with the
# connection id, an announce with a wrong connection id, and a 3-byte packet; prints each reply's message.
import socket, struct, sys
a = ("127.0.0.1", int(sys.argv[1]))
s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
s.settimeout(4)
def rx():
    try:
        return s.recvfrom(2048)[0]
    except OSError as e:
        return repr(e)
def ann(c):
    return (struct.pack(">QII", c, 1, 7) + b"\x00" * 20 + b"-qB00000000000000001"
            + struct.pack(">QQQiIIiH", 0, 0, 0, 0, 0, 0, -1, 6881))
s.sendto(struct.pack(">QII", 0x41727101980, 0, 9), a)
r = rx()
cid = struct.unpack(">Q", r[8:16])[0]
for label, pkt in (("announce, not whitelisted", ann(cid)), ("announce, wrong connection id", ann(cid ^ 0xFF)), ("3-byte packet", b"\x00\x01\x02")):
    s.sendto(pkt, a)
    r = rx()
    print("--- UDP " + label)
    print(r[8:].decode(errors="replace") if isinstance(r, bytes) else r)
