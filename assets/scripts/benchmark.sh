#! /usr/bin/env bash

set -euo pipefail

USER="$1"
APP_PATH="$2"
APPLICATION_DIR="${APP_PATH##*/}"
WORKER_IP="$3"
LUA_FILE="output.lua"

VIRTUAL_USERS="$4"
TIMEOUT="$5"

WARMUP_PAUSE="$6"
WARMUP_DURATION="$7"
WARMUP_RPS="$8"
RECORDS="$9"
PROFILE_NAME="${10}"

(
    script_path=$(dirname -- "${BASH_SOURCE[0]}")
    script_path=$(readlink -f -- "${script_path}")
    cd "$script_path"
    LUA_PATH="$script_path/$LUA_FILE"
    BENCHMARK_RUN="$script_path/benchmarks/${PROFILE_NAME::-4}"
    if [ -d "$BENCHMARK_RUN" ]; then
        TS=$(date +%s)
        BACKUP="$BENCHMARK_RUN-bck-$TS"
        printf "directory %s already exists! Backing up to %s!\n" "$BENCHMARK_RUN" "$BACKUP"
        mv "$BENCHMARK_RUN" "$BACKUP"
    fi
    mkdir -p "$BENCHMARK_RUN"
    CONFIG_FILE="$BENCHMARK_RUN/config.yml"
    printf "Saving benchmark configuration to %s\n" "$CONFIG_FILE" 1>&2
    touch "$CONFIG_FILE"
    printf "virtual_users: %d\n" "$VIRTUAL_USERS" >"$CONFIG_FILE"
    {
        printf "timeout: %d\n" "$TIMEOUT"
        printf "warmup_duration: %d\n" "$WARMUP_DURATION"
        printf "warmup_rps: %d\n" "$WARMUP_RPS"
        printf "warmup_pause: %d\n" "$WARMUP_PAUSE"
        printf "records: %s\n" "$RECORDS"
    } >>"$CONFIG_FILE"

    ssh "$USER@$WORKER_IP" '
cd $HOME/'"$APPLICATION_DIR"'
bash init.sh '"$RECORDS"' init-data'

    echo "[INFO] Starting application"
    ssh "$USER@$WORKER_IP" 'cd '"$APPLICATION_DIR"' && mkdir -p metrics && PROMETHEUS_UID="$(id -u)" PROMETHEUS_GID="$(id -g)" docker compose up --build --detach --force-recreate --wait --quiet-pull'
    echo "[INFO] Successfully started application"
    echo "[INFO] Starting benchmark run"
    cd "../load_generator"
    PROFILE="$PWD/profiles/$PROFILE_NAME"
    LUA_PATH="$LUA_PATH" BENCHMARK_RUN="$BENCHMARK_RUN" PROFILE="$PROFILE" VIRTUAL_USERS="$VIRTUAL_USERS" TIMEOUT="$TIMEOUT" WARMUP_PAUSE="$WARMUP_PAUSE" WARMUP_DURATION="$WARMUP_DURATION" WARMUP_RPS="$WARMUP_RPS" docker compose up --build --abort-on-container-exit --force-recreate
    echo "[INFO] Finished benchmark run"
    echo "[INFO] Stopping application"
    ssh "$USER@$WORKER_IP" 'cd '"$APPLICATION_DIR"' && PROMETHEUS_UID="$(id -u)" PROMETHEUS_GID="$(id -g)" docker compose down'
    rm "$script_path/$LUA_FILE"
    echo "[INFO] Saving collected metrics"
    ssh "$USER@$WORKER_IP" 'cd '"$APPLICATION_DIR"' && tar -czf metrics.tar.gz metrics'
    (
        cd "$BENCHMARK_RUN"
        sftp "$USER@$WORKER_IP" <<EOF
get "$APPLICATION_DIR/metrics.tar.gz"
quit
EOF
    )
    echo "[INFO] Successfully saved collected metrics"
)
