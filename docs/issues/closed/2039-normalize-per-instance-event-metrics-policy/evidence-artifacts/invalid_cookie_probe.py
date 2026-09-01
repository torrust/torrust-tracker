#!/usr/bin/env python3
"""Manual M3 probe: prove invalid UDP connection IDs trigger a shared IP ban.

Run against the metrics-disabled UDP listener from fixed-port-manual.toml:

    python3 invalid_cookie_probe.py 127.0.0.1 17093

The script sends 11 invalid announce requests from one UDP socket. Each must
receive a UDP error response. The twelfth request must time out because the
shared ban service has banned that source IP.
"""

import socket
import struct
import sys

ERROR_ACTION = 3
INVALID_CONNECTION_ID = 0
REQUEST_ACTION = 1
RESPONSE_TIMEOUT_SECONDS = 1


def invalid_announce(transaction_id: int, port: int) -> bytes:
    # cspell:disable
    packed = struct.pack(
        ">QII20s20sQQQIIIiH",
        INVALID_CONNECTION_ID,
        REQUEST_ACTION,
        transaction_id,
        bytes(20),
        bytes(20),
        0,
        0,
        0,
        2,
        0,
        0,
        1,
        port,
    )
    # cspell:enable
    return packed


def expect_error_response(client: socket.socket, transaction_id: int) -> None:
    response, _ = client.recvfrom(2048)
    action, response_transaction_id = struct.unpack(">II", response[:8])
    if action != ERROR_ACTION or response_transaction_id != transaction_id:
        raise RuntimeError(f"unexpected response for transaction {transaction_id}: {response!r}")


def main() -> None:
    if len(sys.argv) != 3:
        raise SystemExit(f"usage: {sys.argv[0]} <host> <port>")

    endpoint = (sys.argv[1], int(sys.argv[2]))
    with socket.socket(socket.AF_INET, socket.SOCK_DGRAM) as client:
        client.settimeout(RESPONSE_TIMEOUT_SECONDS)
        client.connect(endpoint)
        source_port = client.getsockname()[1]

        for transaction_id in range(1, 12):
            client.send(invalid_announce(transaction_id, source_port))
            expect_error_response(client, transaction_id)

        client.send(invalid_announce(12, source_port))
        try:
            client.recv(2048)
        except TimeoutError:
            print("PASS: the twelfth invalid request timed out after shared ban enforcement")
            return

        raise RuntimeError("expected the twelfth invalid request to be banned")


if __name__ == "__main__":
    main()
