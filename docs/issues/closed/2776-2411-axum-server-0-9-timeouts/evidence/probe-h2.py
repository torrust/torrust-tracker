import socket, ssl, sys, time
# usage: probe-h2.py <label> <port> <plain|tls> ; HTTP/2 request then silence; client never ACKs PINGs.
label, port, mode = sys.argv[1], int(sys.argv[2]), sys.argv[3]
s = socket.create_connection(("127.0.0.1", port))
if mode == "tls":
    ctx = ssl.create_default_context(); ctx.check_hostname = False; ctx.verify_mode = ssl.CERT_NONE
    ctx.set_alpn_protocols(["h2"])
    s = ctx.wrap_socket(s, server_hostname="localhost")
    print(f"{label}: alpn={s.selected_alpn_protocol()}")
hdr = bytes([0x82, 0x84, 0x86, 0x01, 9]) + b"localhost"
# The request is `GET /` (HPACK static entries), which every listener answers, with a 404 on the trackers.
req = b"PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n" + bytes([0,0,0,4,0,0,0,0,0]) + len(hdr).to_bytes(3,"big") + bytes([1,0x05,0,0,0,1]) + hdr
s.sendall(req)
t0 = time.time(); s.settimeout(25)
try:
    while True:
        d = s.recv(4096)
        if not d:
            print(f"{label}: closed by server after {time.time()-t0:.1f}s from request"); break
except socket.timeout:
    print(f"{label}: STILL OPEN after 25.0s (client gave up)")
except (ConnectionResetError, ssl.SSLError) as e:
    print(f"{label}: closed with error after {time.time()-t0:.1f}s: {type(e).__name__}")
