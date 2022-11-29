#!/bin/sh
set -e

PWD=$(PWD)
DIR=$(basename "$PWD")
YEAR="${YEAR:-$DIR}"
DAY="${DAY:-$(date +"%d")}"

if [ ! -d "$DAY/src" ]
then
    echo "✨ Making directory for day $DAY"
    mkdir -p "$DAY/src"
    sed "s/template/day$DAY/g" template/Cargo.toml > "$DAY/Cargo.toml"
    cp template/src/main.rs "$DAY/src/"
fi

if [ ! -f "$DAY/input" ]
then
    # Be polite and only attempt to download available inputs
    TODAY=$(date +"%Y%m%d")
    TARGET="${YEAR}12${DAY}"
    if [ $TODAY -ge $TARGET ]
    then
        COOKIE=$(cat ~/.aoc-cookie)
        echo "⬇️  Downloading input for day $DAY"
        curl "https://adventofcode.com/$YEAR/day/$DAY/input" -H "Cookie: $COOKIE" > "$DAY/input"
    else
        echo "⚠️  $YEAR-$DAY is not yet available; skipping download"
    fi
fi

# Sanity-checking on the input
if [ -f "$DAY/input" ]
then
    if grep "Please don't repeatedly request this endpoint" "$DAY/input" > /dev/null
    then
        # This shouldn't happen, due to the date check above
        echo "❌ Download failed due to rate-limiting on the server"
    elif grep "Puzzle inputs differ by user" "$DAY/input" > /dev/null
    then
        echo "❌ Login failed; perhaps your cookie is stale?"
    fi
fi
