#!/bin/sh
set -e

DAY="${DAY:-$(date +"%d")}"
BOLD=$(tput bold)
NORMAL=$(tput sgr0)

run_file() {
  echo "${BOLD}Running day${DAY} on ${1}${NORMAL}"
  cargo run -q --bin "day${DAY}" < "${1}"
}

run_input() {
  run_file "${DAY}/input"
}

run_example() {
  run_file ./example
}

run_paste() {
    echo "${BOLD}Running on paste buffer${NORMAL}"
    pbpaste | cargo run -q --bin "day${DAY}"
}

if [ $# -eq 0 ]; then
    run_input
    exit 0
fi

while getopts iep flag
do
    case "${flag}" in
        i) run_input;;
        e) run_example;;
        p) run_paste;;
        *) exit 1;;
    esac
done
