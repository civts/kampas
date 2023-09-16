#! /usr/bin/env bash

declare -A colors
colors["blue"]="\x1b[34m"
colors["magenta"]="\x1b[35m"
colors["green"]="\x1b[32m"
colors["red"]="\x1b[31m"
colors["yellow"]="\x1b[33m"
colors["cyan"]="\x1b[36m"

execute_with_label() {
    label=$1
    command=$2
    color=$3

    # (bash -c "$command" 2>&1 | sed "s/.*/$color[$label]\x1b[0m &/"; echo -e "$color-------------$label done--------------\x1b[0m") &
    (bash -c "$command" 2> >(sed "s/^/$color[$label]${colors['red']} &/" | sed 's/$/\x1b[0m &/' >&2) | sed "s/^/$color[$label]\x1b[0m &/"; echo -e "$color-------------$label done--------------\x1b[0m") &
}

execute_with_label "surreal" "cd kampas; docker compose down; docker compose up" "${colors["yellow"]}"
execute_with_label "redis" "cd frontend; docker compose down; docker compose up" "${colors["green"]}"
execute_with_label "backend" "source ./kampas/.env; cargo watch -C kampas -x run" "${colors["blue"]}"
execute_with_label "frontend" "sleep 4; cd frontend; npm run dev" "${colors["magenta"]}" 

kill_background_jobs() {
    echo "Terminating"
    if [[ $(jobs -p) ]]; then
        kill $(jobs -p) 2>/dev/null
    fi
}

trap 'kill_background_jobs' SIGINT

wait
