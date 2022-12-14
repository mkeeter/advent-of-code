#!/bin/sh
set -e

DAY="${DAY:-$(date +"%d")}"
BOLD=$(tput bold)
NORMAL=$(tput sgr0)

run_file() {
    echo "${BOLD}Running day${DAY} on ${1}${NORMAL} (${PROFILE})"
    cargo run -q --profile "$PROFILE" --bin "day${DAY}" < "${1}"
}

run_input() {
    run_file "${DAY}/input"
}

run_example() {
    run_file ./example
}

run_paste() {
    echo "${BOLD}Running on paste buffer${NORMAL} (${PROFILE})"
    pbpaste | cargo run -q --profile "$PROFILE" --bin "day${DAY}"
}

help() {
   echo "Usage: ./run.sh [-iepr]"
   echo "  -i: Run on today's input"
   echo "  -e: Run on ./example"
   echo "  -p: Run on paste buffer"
   echo "  -r: Use release mode (default is dev)"
}


PROFILE="dev"
TARGET=INPUT
while getopts ieprh flag
do
    case "${flag}" in
        i) TARGET=INPUT;;
        e) TARGET=EXAMPLE;;
        p) TARGET=PASTE;;
        h) TARGET=HELP;;
        r) PROFILE="release";;
        *) exit 1;;
    esac
done

case $TARGET in
    PASTE)
        run_paste
        break;;
    INPUT)
        run_input
        break;;
    EXAMPLE)
        run_example
        break;;
    *)
        help
        break;;
esac
