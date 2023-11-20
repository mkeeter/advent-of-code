#!/bin/sh
set -e

PWD=$(PWD)
DIR=$(basename "$PWD")
YEAR="${YEAR:-$DIR}"
DAY="${DAY:-$(date +"%d")}"

echo " 📅  Preparing $YEAR-$DAY"
if [ ! -d "$DAY/src" ]
then
    echo " ✨ Making directory for day $DAY"
    mkdir -p "$DAY/src"
    sed "s/template/day$DAY/g" template/Cargo.toml > "$DAY/Cargo.toml"
    cp template/src/main.rs "$DAY/src/"
    cargo check -p "day$DAY" &> /dev/null
    git add "$DAY" Cargo.lock
fi

if [ ! -f "$DAY/input" ]
then
    # Be polite and only attempt to download available inputs
    TODAY=$(date +"%Y%m%d")
    TARGET="${YEAR}12${DAY}"
    if [ "$TODAY" -ge "$TARGET" ]
    then
        COOKIE=$(cat ~/.aoc-cookie)
        DAY_SHORT=$(echo "$DAY" | sed "s/^0*//g")
        echo " ⬇️   Downloading input for day $DAY_SHORT"
        curl -s "https://adventofcode.com/$YEAR/day/$DAY_SHORT/input" \
            --cookie "session=$COOKIE" \
            --user-agent "https://github.com/mkeeter/advent-of-code/blob/master/2022/download.sh by matt.j.keeter@gmail.com" \
            > "$DAY/input"
    else
        echo " ⚠️   $YEAR-$DAY is not yet available; skipping download"
    fi
else
    echo " 🔎  $DAY/input already exists; skipping download"
fi

# Sanity-checking on the input
if [ -f "$DAY/input" ]
then
    if grep "Please don't repeatedly request this endpoint" "$DAY/input" > /dev/null
    then
        # This shouldn't happen, due to the date check above
        echo " ❌  Download failed due to rate-limiting on the server"
    elif grep "Puzzle inputs differ by user" "$DAY/input" > /dev/null
    then
        echo " ❌  Login failed; perhaps your cookie is stale?"
    else
        echo " ✅  Ready to go!"
    fi
fi
