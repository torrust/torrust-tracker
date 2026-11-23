#!/usr/bin/env bash
# Usage: reproduce.sh <tracker-binary> <work-dir>. Reproduces the EPIC #2993 inventory cases. Test values only.
set -u
BIN=$1; W=$2; mkdir -p "$W"; cd "$W" || exit 1
strip(){ sed 's/\x1b\[[0-9;]*m//g'; }
hdr='[metadata]
app = "torrust-tracker"
purpose = "configuration"
schema_version = "3.0.0"
[logging]
trace_filter = "debug"
trace_style = "full"
'
start(){ ("$BIN" --config-toml-path "$1" > "$2" 2>&1 & echo $! > "$2.pid"); for _ in $(seq 1 30); do curl -s -o /dev/null "http://127.0.0.1:$3/api/health_check" && return; sleep 1; done; }
stop(){ kill "$(cat "$1.pid")" 2>/dev/null; sleep 1; }
H='Authorization: Bearer TestAdminToken0001'
IH='info_hash=%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00&peer_id=-qB00000000000000001&port=17548&left=0'

echo "### V1-V5 private tracker, sqlite"
printf '%s[core]\nlisted = false\nprivate = true\n[core.database]\ndriver = "sqlite3"\npath = "%s/private.db"\n[http_api]\nbind_address = "127.0.0.1:41212"\n[http_api.access_tokens]\nadmin = "TestAdminToken0001"\n[[http_trackers]]\nbind_address = "127.0.0.1:47070"\n[health_check_api]\nbind_address = "127.0.0.1:41313"\n' "$hdr" "$W" > private.toml
rm -f private.db*; start private.toml private.log 41212
echo "--- V1 announce, unregistered key"; curl -s "http://127.0.0.1:47070/announce/TestPeerKey000000000000000000001?$IH"; echo
echo "--- V1 announce, no key"; curl -s "http://127.0.0.1:47070/announce?$IH"; echo
echo "--- V1 announce, 14-character key"; curl -s "http://127.0.0.1:47070/announce/ShortTestKey01?$IH"; echo
echo "--- V2 REST add 14-character key"; curl -s -X POST -H "$H" -H 'content-type: application/json' -d '{"key":"ShortTestKey01","seconds_valid":100}' -w ' [%{http_code}]\n' http://127.0.0.1:41212/api/v1/keys
echo "--- V2 REST delete malformed key"; curl -s -X DELETE -H "$H" -w ' [%{http_code}]\n' http://127.0.0.1:41212/api/v1/key/BadTestKey
echo "--- V3 REST wrong token in query"; curl -s -w ' [%{http_code}]\n' "http://127.0.0.1:41212/api/v1/stats?token=WrongTestToken0001"
echo "--- V4 REST generate key (reply not shown)"; curl -s -o gen.json -X POST -H "$H" -w '[%{http_code}]\n' http://127.0.0.1:41212/api/v1/key/3600
echo "--- V5 REST duplicate key (sqlite)"; for _ in 1 2; do curl -s -X POST -H "$H" -H 'content-type: application/json' -d '{"key":"TestPeerKey000000000000000000002","seconds_valid":100}' -w ' [%{http_code}]\n' http://127.0.0.1:41212/api/v1/keys; done
echo "--- V5 REST malformed JSON"; curl -s -X POST -H "$H" -H 'content-type: application/json' -d '{bad' -w ' [%{http_code}]\n' http://127.0.0.1:41212/api/v1/keys
echo "--- V5 REST unknown path"; curl -s -o /dev/null -X GET -H "$H" -w '[%{http_code}]\n' http://127.0.0.1:41212/api/v1/key/abc/extra
stop private.log
echo "--- log: uri fields holding a test key or token"; strip < private.log | grep -o -E 'uri=[^ ]*(TestPeerKey|ShortTestKey|BadTestKey|WrongTestToken)[^ ]*' | sort -u
echo "--- log: lines holding the admin token sent in the header: $(strip < private.log | grep -c TestAdminToken0001) (the startup configuration dump prints it as ***)"
GK=$(sed -E 's/.*"key":"([^"]+)".*/\1/' gen.json); echo "--- log: 'Generated key' lines: $(strip < private.log | grep -c 'Generated key'); lines holding the generated key: $(strip < private.log | grep -c "$GK")"
strip < private.log | grep 'Generated key' | sed "s/$GK/<the generated key>/" | cut -c1-200
echo "--- log: lines for the 500 reply"; strip < private.log | grep -E 'status_code=500' | head -1 | cut -c1-330

echo; echo "### V6 UDP, public listed tracker, sqlite"
printf '%s[core]\nlisted = true\nprivate = false\n[core.database]\ndriver = "sqlite3"\npath = "%s/public.db"\n[http_api]\nbind_address = "127.0.0.1:41212"\n[http_api.access_tokens]\nadmin = "TestAdminToken0001"\n[[udp_trackers]]\nbind_address = "127.0.0.1:46969"\n[[http_trackers]]\nbind_address = "127.0.0.1:47070"\n[health_check_api]\nbind_address = "127.0.0.1:41313"\n' "$hdr" "$W" > public.toml
rm -f public.db*; start public.toml public.log 41212
python3 -I "$(dirname "$0")/udp_probe.py" 46969
echo "--- HTTP announce, not whitelisted"; curl -s "http://127.0.0.1:47070/announce?$IH"; echo
stop public.log
echo "--- private mode refuses UDP:"; strip < private.log | grep -o 'Could not start UDP trackers.*' | head -1

echo; echo "### V7 configuration errors"
db='[core]\nlisted = false\nprivate = false\n[core.database]\ndriver = "mysql"\nhost = "127.0.0.1"\nuser = "db_user"\ndatabase = "torrust_tracker"\n'
printf "%s${db}port = 3306\npassword = 987654321\n" "$hdr" > pw-int.toml
printf "%s${db}port = 3306\npassword = \"x\"\n[http_api]\nbind_address = \"127.0.0.1:0\"\n[http_api.access_tokens]\nadmin = 424242\n" "$hdr" > token-int.toml
# shellcheck disable=SC2016  # the backticks are literal sed text
for n in pw-int token-int; do echo "--- $n"; timeout 20 "$BIN" --config-toml-path $n.toml 2>&1 | strip | grep '^Tracker startup failed' | sed 's/.*explicit configuration file `[^`]*`: //'; done

echo; echo "### V8 database server messages (MySQL 8.0, PostgreSQL 16)"
docker run -d --rm --name ev2993-mysql -e MYSQL_ROOT_PASSWORD=TestDbPassword01 -e MYSQL_DATABASE=torrust_tracker -p 127.0.0.1:33061:3306 mysql:8.0 >/dev/null
docker run -d --rm --name ev2993-pg -e POSTGRES_PASSWORD=TestDbPassword01 -e POSTGRES_DB=torrust_tracker -p 127.0.0.1:54321:5432 postgres:16 >/dev/null
for _ in $(seq 1 60); do docker exec ev2993-mysql mysql -h127.0.0.1 -uroot -pTestDbPassword01 -e 'select 1' >/dev/null 2>&1 && break; sleep 2; done
for _ in $(seq 1 30); do docker exec ev2993-pg pg_isready -q && break; sleep 2; done; sleep 3
mk(){ printf '%s[core]\nlisted = false\nprivate = true\n[core.database]\ndriver = "%s"\nhost = "127.0.0.1"\nport = %s\nuser = "%s"\npassword = "%s"\ndatabase = "torrust_tracker"\n[http_api]\nbind_address = "127.0.0.1:41212"\n[http_api.access_tokens]\nadmin = "TestAdminToken0001"\n[health_check_api]\nbind_address = "127.0.0.1:41313"\n' "$hdr" "$1" "$2" "$3" "$4" > "$5"; }
mk mysql 33061 db_user WrongTestPassword mysql-bad.toml; mk postgresql 54321 db_user WrongTestPassword pg-bad.toml
for n in mysql-bad pg-bad; do echo "--- $n (wrong credentials)"; timeout 40 "$BIN" --config-toml-path $n.toml 2>&1 | strip | grep '^Tracker startup failed' | sed 's/.*schema permissions: //'; done
mk mysql 33061 root TestDbPassword01 mysql.toml; mk postgresql 54321 postgres TestDbPassword01 pg.toml
for n in mysql pg; do echo "--- $n duplicate key"; start $n.toml $n.log 41212; for _ in 1 2; do curl -s -X POST -H "$H" -H 'content-type: application/json' -d '{"key":"TestPeerKey000000000000000000003","seconds_valid":100}' -w ' [%{http_code}]\n' http://127.0.0.1:41212/api/v1/keys; done; stop $n.log; echo "log lines holding the key: $(strip < $n.log | grep -c TestPeerKey000000000000000000003)"; strip < $n.log | grep TestPeerKey000000000000000000003 | cut -c1-330; done
docker stop ev2993-mysql ev2993-pg >/dev/null
