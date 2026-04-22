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
