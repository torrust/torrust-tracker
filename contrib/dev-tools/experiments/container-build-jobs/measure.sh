#!/bin/bash
# Measure peak memory and wall time of the release image build for one CARGO_BUILD_JOBS value.
#
# Usage: measure.sh <jobs> [<min-available-gib>]
#
# Safety: runs ONE uncached build, refuses to start while another container build or root-owned
# `rustc` (a Docker/BuildKit build) is running, samples MemAvailable every 2 seconds and cancels the
# build (by PID of the `docker build` client) when MemAvailable drops below the threshold
# (default 8 GiB). Logs go to ~/.cache (not /tmp: it is RAM and wiped on reboot).
set -u

jobs="${1:?usage: measure.sh <jobs> [<min-available-gib>]}"
min_gib="${2:-8}"
min_kb=$((min_gib * 1024 * 1024))
log_dir="${HOME}/.cache/torrust-tracker-build-jobs"
mkdir -p "${log_dir}"
stamp="$(date +%Y%m%d-%H%M%S)"
samples="${log_dir}/jobs-${jobs}-${stamp}.samples.tsv"
build_log="${log_dir}/jobs-${jobs}-${stamp}.build.log"
summary="${log_dir}/jobs-${jobs}-${stamp}.summary.txt"

if pgrep -af 'docker(-buildx)?( buildx)? build' | grep -v "$$" | grep -q .; then
    echo "another docker build is running; refusing to start" >&2
    exit 2
fi
if pgrep -u root -x rustc >/dev/null; then
    echo "a root-owned rustc (container build) is running; refusing to start" >&2
    exit 2
fi

mem_available_kb() { awk '/^MemAvailable:/ {print $2}' /proc/meminfo; }

baseline_kb="$(mem_available_kb)"
echo -e "epoch\tmem_available_kb\trustc_count\trustc_rss_kb" >"${samples}"

start="$(date +%s)"
docker build --no-cache --progress=plain --target release \
    --build-arg "CARGO_BUILD_JOBS=${jobs}" \
    --tag "torrust-tracker:jobs-${jobs}" --file Containerfile . >"${build_log}" 2>&1 &
build_pid=$!
echo "build pid ${build_pid}, baseline MemAvailable ${baseline_kb} kB" | tee "${summary}"

# If this script itself is interrupted (Ctrl-C, or killed by someone else), stop the background
# build too: otherwise it keeps consuming memory with no watchdog left to cancel it.
trap 'echo "measure.sh interrupted; cancelling build pid ${build_pid}" | tee -a "${summary}" >&2
      kill -TERM "${build_pid}" 2>/dev/null
      wait "${build_pid}" 2>/dev/null
      exit 130' INT TERM

aborted=0
while kill -0 "${build_pid}" 2>/dev/null; do
    avail="$(mem_available_kb)"
    read -r count rss < <(ps -C rustc -o user=,rss= | awk '$1=="root" {n++; s+=$2} END {print n+0, s+0}')
    echo -e "$(date +%s)\t${avail}\t${count}\t${rss}" >>"${samples}"
    if [ "${avail}" -lt "${min_kb}" ]; then
        echo "WATCHDOG: MemAvailable ${avail} kB < ${min_kb} kB; cancelling build pid ${build_pid}" | tee -a "${summary}"
        kill -TERM "${build_pid}"
        aborted=1
        break
    fi
    sleep 2
done
wait "${build_pid}"
status=$?
end="$(date +%s)"

{
    echo "jobs=${jobs} exit=${status} aborted=${aborted} wall_seconds=$((end - start))"
    awk -F'\t' -v base="${baseline_kb}" 'NR>1 {
        if (min=="" || $2<min) min=$2
        if ($3>max_count) max_count=$3
        # Track peak RSS and the rustc count of that SAME sample (not an independent column
        # maximum, which can come from a different, lower-memory moment, for example a brief
        # process-count spike at a stage handoff before the new processes have grown).
        if ($4>max_rss) { max_rss=$4; count_at_peak_rss=$3 }
    } END {
        printf "baseline_available_gib=%.1f min_available_gib=%.1f peak_used_delta_gib=%.1f peak_rustc_rss_gib=%.1f rustc_count_at_peak_rss=%d max_concurrent_rustc_count=%d\n",
            base/1048576, min/1048576, (base-min)/1048576, max_rss/1048576, count_at_peak_rss, max_count
    }' "${samples}"
} | tee -a "${summary}"
exit "${status}"
