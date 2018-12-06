#!/usr/bin/env bash

readonly YEAR=2018

function usage {
	echo "Usage: $0 [DAYNR] [DAYTITLE]"
	exit 1
}

# Check the number of arguments
if [[ "$#" -ne 2 ]]; then
	echo "Wrong number of parameters"
	usage
fi

# Check that the first argument is a number
if ! [[ "$1" =~ ^[0-9]+$ ]]; then
	echo "First argument should be the day number"
	usage
fi

# Check that there is no directory for this day
for each in $( ls -d day-$1-* 2> /dev/null ); do
	echo "There already exists a directory for day $1"
	usage
done

# Convert second argument (title) to directory name
DIR=day-$1-$(echo "$2" | tr '[:upper:]' '[:lower:]' | tr ' ' '-' | cat)

# Copy template to directory
cp -r template "$DIR"

# Convert occurances of 20xx correct year
fd -t f -x sed -i "s/20xx/$YEAR/g" "{}" \; ".*" "$DIR"
# Convert occurances of day-x and day_x to correct day
fd -t f -x sed -i "s/day\(-\|_\)x/day\1$1/g" "{}" \; ".*" "$DIR"

# Add directory as member to workspace
MEMBER="\ \ \ \ \"$DIR\","
sed -i "/^\]$/i $MEMBER" Cargo.toml
