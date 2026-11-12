import socket, ssl, sys, time
# usage: probe.py <label> <port> <mode: plain|tls|persistent> [path]
label, port, mode = sys.argv[1], int(sys.argv[2]), sys.argv[3]
s = socket.create_connection(("127.0.0.1", port))
if mode == "tls":
    ctx = ssl.create_default_context(); ctx.check_hostname = False; ctx.verify_mode = ssl.CERT_NONE
    s = ctx.wrap_socket(s, server_hostname="localhost")
t0 = time.time()
if mode == "persistent":
    s.sendall(f"GET {sys.argv[4]} HTTP/1.1\r\nHost: localhost\r\n\r\n".encode())
    first = s.recv(4096).split(b"\r\n")[0]
    print(f"{label}: first response: {first.decode()}")
    t0 = time.time()
s.settimeout(25)
try:
    while True:
        d = s.recv(4096)
        if not d:
            print(f"{label}: closed by server after {time.time()-t0:.1f}s"); break
        print(f"{label}: received {len(d)} bytes (ignored)")
except socket.timeout:
    print(f"{label}: STILL OPEN after 25.0s (client gave up)")
except (ConnectionResetError, ssl.SSLError) as e:
    print(f"{label}: closed with error after {time.time()-t0:.1f}s: {type(e).__name__}")
