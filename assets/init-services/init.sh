#! /usr/bin/env bash

_print_usage() {
	printf "Usage:\n\t %s <seed_count> -- initialize services\n" "$0" >&2
}

_assert_is_number() {
	re="^[0-9]+$"
	if ! [[ "$1" =~ $re ]]; then
		printf "error: %s is not a number\n" "$1" >&2
		_print_usage
		exit 1
	fi
}

_create_env_file() {
	printf "MG_SEED_COUNT=%s\n" "$1" >".env"
	uid="$(id -u)"
	gid="$(id -g)"
	printf "PROMETHEUS_UID=%s\nPROMETHEUS_GID=%s\n" "$uid" "$gid" >>".env"
}

_delete_all_volumes() {
	docker compose down --volumes
}

_wait_for_pid() {
	pid="$1"
	wait "$pid"
	return_code="$?"
	return "$return_code"
}

_stop_compose() {
	docker compose down
}

_wait_for_docker_service() {
	local service
	service="$1"
	docker compose wait "$service" >/dev/null 2>&1
	docker compose rm -f "$service" >/dev/null 2>&1
}
set -uo pipefail
(
	script_path=$(dirname -- "${BASH_SOURCE[0]}")
	script_path=$(readlink -f -- "${script_path}")
	cd "${script_path}" || exit 1

	seed_count="$1"

	_assert_is_number "$seed_count"

	_create_env_file "$seed_count"

	_delete_all_volumes

	service_names=()
	while read -r line; do
		if [ -z "$line" ]; then continue; fi
		service_names+=("$line")
	done < <(tr -d "\r" <init-services.conf)

	tar_name=""
	if [ "$#" -eq 2 ]; then
		tar_name="$2-$1"
	fi

	project_name=$(basename "$PWD")
	volume_names=$(docker compose config --volumes)
	IFS=$'\n' read -rd '' -a volumes <<<"$volume_names"
	n_volumes="${#volumes[@]}"
	backups=0
	for ((i = 0; i < "$n_volumes"; i++)); do
		volume="${project_name}_${volumes[$i]}"
		archive="$tar_name-$volume.tar.gz"
		if [ -f "$archive" ]; then
			echo "[INFO] Found backup file"
			echo "[INFO] Extracting archive"
			docker compose create "${service_names[0]}"
			docker run --rm -v "$volume:/backup-volume" -v "$PWD:/backup" busybox tar -xzf "/backup/$archive" -C "/backup-volume"
			backups=1
		fi
	done
	if [ "$backups" -eq 1 ]; then
		exit 0
	fi

	n_services="${#service_names[@]}"
	pids=()
	for ((i = 0; i < "$n_services"; i++)); do
		service_name="${service_names[$i]}"
		docker compose up --detach --no-recreate "$service_name" >/dev/null 2>&1
		_wait_for_docker_service "$service_name" &
		pids+=("$!")
	done

	exit_code=0

	for ((i = 0; i < "$n_services"; i++)); do
		pid="${pids[$i]}"
		_wait_for_pid "$pid"
		return_code=$?
		if [ "$return_code" -ne 0 ]; then
			printf "[ERROR] Command \`%s\` exited with error status code %d\n" "${service_names[$i]}" "$return_code"
			exit_code=1
		fi
	done

	_stop_compose
	if [ "$tar_name" ]; then
		for ((i = 0; i < "$n_volumes"; i++)); do
			volume="${project_name}_${volumes[$i]}"
			archive="$tar_name-$volume.tar.gz"
			docker run --rm -v "$volume:/backup-volume" -v "$PWD:/backup" busybox tar -czf "/backup/$archive" -C "/backup-volume" .
		done
	fi

	exit "$exit_code"
)
