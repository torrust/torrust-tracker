## qBittorrent Debugging

These scripts help debug the qBittorrent-based E2E workflow without running the
entire Rust runner.

Available scripts:

- `qbittorrent-login-probe.sh`: starts an isolated qBittorrent 5.1.4 container,
  prepares a `/config` mount, and probes WebUI authentication behavior. Use it
  to debug browser access, CSRF header handling, Host validation, and temporary
  password behavior.
- `check-qbittorrent-e2e-compose.sh`: validates and brings up the full compose
  stack to confirm container startup, port publishing, and image wiring before
  debugging orchestration logic in Rust.

Suggested workflow:

1. Use `qbittorrent-login-probe.sh` when the WebUI itself is failing.
2. Use `check-qbittorrent-e2e-compose.sh` when the isolated UI works but the
   full stack still fails.
3. Run the Rust `qbittorrent_e2e_runner` only after the smaller debugging steps
   pass.

## Troubleshooting

### WebUI returns Unauthorized in browser

Symptom:

- Opening the leecher WebUI on the published host port (for example,
  `http://127.0.0.1:32867`) shows Unauthorized.
- Browser private mode does not help.
- API login to that host port can return `401 Unauthorized` even with valid
  credentials.

Observed cause:

- qBittorrent accepts authentication only when the request Host/Origin/Referer
  match `localhost:8080` in this setup.
- The E2E stack publishes container WebUI port `8080` to a random host port
  (for example, `32867`), which can trigger this mismatch.

How to verify:

1. Confirm the leecher port mapping.
2. Compare login responses with and without host header override.

   docker compose -f ./compose.qbittorrent-e2e.yaml -p <project> port qbittorrent-leecher 8080
   curl -i -X POST http://127.0.0.1:<host-port>/api/v2/auth/login \
    --data 'username=admin&password=adminadmin'
   curl -i -X POST http://127.0.0.1:<host-port>/api/v2/auth/login \
    -H 'Host: localhost:8080' \
    -H 'Referer: http://localhost:8080' \
    -H 'Origin: http://localhost:8080' \
    --data 'username=admin&password=adminadmin'

Expected result:

- First login can return `401 Unauthorized`.
- Second login should return `200 OK` with body `Ok.`

Important:

- Do not treat HTTP status code alone as success. qBittorrent can return
  `200 OK` with body `Fails.` when credentials are wrong.
- Successful login response body is exactly `Ok.`

Workaround for manual browser inspection:

1. Forward local port `8080` to the published leecher host port.

   socat TCP-LISTEN:8080,reuseaddr,fork TCP:127.0.0.1:<host-port>

2. Open `http://localhost:8080`.
3. Log in with `admin` / `torrust-e2e-pass`.
4. Stop the forwarder with `Ctrl+C` when done.

Notes:

- If needed, install socat with your system package manager (for example,
  `sudo apt-get install -y socat`).
- This is a debugging workaround for manual inspection. Keep using the runner
  logs as the source of truth for automated pass/fail checks.

### Repeated login attempts lead to temporary IP ban

Symptom:

- Login requests start returning `403 Forbidden`.
- Response body contains: `Your IP address has been banned after too many
failed authentication attempts.`

Observed cause:

- Multiple failed login attempts from the same client IP quickly trigger
  qBittorrent WebUI protection.

How to verify safely:

1. Recreate a fresh stack before re-testing auth.
2. Make one login attempt only.
3. Check both status and body:
   - success: `200 OK` + `Ok.`
   - wrong credentials: `200 OK` + `Fails.`
   - banned: `403 Forbidden` + ban message above

Recommended practice:

- Prefer one controlled API login check first, then browser login.
- Avoid trying fallback credentials repeatedly on the same running stack.
