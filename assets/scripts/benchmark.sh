#! /usr/bin/env bash

set -euo pipefail

USER="$1"
APP_PATH="$2"
APPLICATION_DIR="${APP_PATH##*/}"
WORKER_IP="$3"
YAML_FILE="output.yml"

BENCHMARK_DURATION="$4"
START_RPS="$5"
END_RPS="$6"
DIRECTOR_THREADS="$7"
VIRTUAL_USERS="$8"
TIMEOUT="$9"

WARMUP_PAUSE="${10}"
WARMUP_DURATION="${11}"
WARMUP_RPS="${12}"
RECORDS="${13}"
PROFILE=""
if [ "$#" -eq 14 ]; then
    PROFILE="${14}"
fi

(
    script_path=$(dirname -- "${BASH_SOURCE[0]}")
    script_path=$(readlink -f -- "${script_path}")
    cd "$script_path"
    YAML_PATH="$script_path/$YAML_FILE"
    if [ -z "$PROFILE" ]; then
        BENCHMARK_RUN="$script_path/benchmarks/$APPLICATION_DIR/$START_RPS-$END_RPS"
    else
        BENCHMARK_RUN="$script_path/benchmarks/$APPLICATION_DIR/$PROFILE"
    fi
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
    printf "duration: %s\n" "$BENCHMARK_DURATION" >"$CONFIG_FILE"
    {
        printf "threads: %d\n" "$DIRECTOR_THREADS"
        printf "virtual_users: %d\n" "$VIRTUAL_USERS"
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
    if [ -z "$PROFILE" ]; then
        YAML_PATH="$YAML_PATH" BENCHMARK_RUN="$BENCHMARK_RUN" START_RPS="$START_RPS" END_RPS="$END_RPS" BENCHMARK_DURATION="$BENCHMARK_DURATION" DIRECTOR_THREADS="$DIRECTOR_THREADS" VIRTUAL_USERS="$VIRTUAL_USERS" TIMEOUT="$TIMEOUT" WARMUP_PAUSE="$WARMUP_PAUSE" WARMUP_DURATION="$WARMUP_DURATION" WARMUP_RPS="$WARMUP_RPS" docker compose up --build --abort-on-container-exit --force-recreate
    else
        PROFILE="$PWD/profiles/$PROFILE"
        YAML_PATH="$YAML_PATH" BENCHMARK_RUN="$BENCHMARK_RUN" PROFILE="$PROFILE" THREADS="$DIRECTOR_THREADS" VIRTUAL_USERS="$VIRTUAL_USERS" TIMEOUT="$TIMEOUT" WARMUP_PAUSE="$WARMUP_PAUSE" WARMUP_DURATION="$WARMUP_DURATION" WARMUP_RPS="$WARMUP_RPS" docker compose -f profile-compose.yml up --build --abort-on-container-exit --force-recreate
    fi
    echo "[INFO] Finished benchmark run"
    echo "[INFO] Stopping application"
    ssh "$USER@$WORKER_IP" 'cd '"$APPLICATION_DIR"' && PROMETHEUS_UID="$(id -u)" PROMETHEUS_GID="$(id -g)" docker compose down'
    rm "$script_path/$YAML_FILE"
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
