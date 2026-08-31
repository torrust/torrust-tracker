# M6 No-Persistence Container Verification

**Date:** 2026-08-30

## Scope

This verification exercised the release image's normal entrypoint with neither
a mounted configuration nor
`TORRUST_TRACKER_CONFIG_OVERRIDE_CORE__DATABASE__DRIVER`. It used isolated
mounted state, log, and configuration directories.

## Commands And Result

```text
docker build --target release --tag torrust-tracker:2107-no-persistence -f Containerfile .

docker run --rm --entrypoint /bin/sh torrust-tracker:2107-no-persistence \
  -c 'test ! -e /usr/share/torrust/default/database/tracker.sqlite3.db \
    && test ! -d /usr/share/torrust/default/database'

docker run --rm --name torrust-2107-no-persistence-final \
  --env USER_ID="$(id -u)" \
  --publish 127.0.0.1:11314:1313 \
  --volume "$PWD/.tmp/2107-container-no-persistence-final/lib:/var/lib/torrust/tracker:rw" \
  --volume "$PWD/.tmp/2107-container-no-persistence-final/log:/var/log/torrust/tracker:rw" \
  --volume "$PWD/.tmp/2107-container-no-persistence-final/etc:/etc/torrust/tracker:rw" \
  torrust-tracker:2107-no-persistence

curl --fail --silent --show-error http://127.0.0.1:11314/health_check
```

The image build, including its embedded full test suite, passed. Startup
installed `tracker.container.no-persistence.toml`; the tracker logged
`"database": null` and started public UDP and HTTP listeners plus the health
API. The health response reported `"status":"Ok"` and healthy UDP and HTTP
checks.

The final image contained neither
`/usr/share/torrust/default/database/tracker.sqlite3.db` nor its database
directory. Its mounted state contained only `etc/tracker.toml`, `lib`, and
`log`; it contained neither `lib/database` nor `lib/database/sqlite3.db`, and
the installed configuration contained no `[core.database]` section. Docker
reported the running container as `healthy`.

## Result

The supported release-image path starts a documented v3 no-persistence tracker
without a database-driver override, a packaged SQLite database, or a
persistence-only database directory. M6, T5, and AC11 are complete.
